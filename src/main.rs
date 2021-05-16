extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

use std::env;

use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use diesel_migrations::embed_migrations;
use dotenv::dotenv;
use tide::http::headers::HeaderValue;
use tide::prelude::*;
use tide::security::{CorsMiddleware, Origin};
use tide::{Body, Error, Request, Response};
use uuid::Uuid;

use std::process::exit;
use todo_in_rust_with_tide::models::Todo;
use todo_in_rust_with_tide::schema::todos::dsl::todos;

#[derive(Debug, Serialize)]
struct Todolist {
    todos: Vec<Todo>,
}

#[derive(Deserialize)]
struct NewRequestTodo {
    title: String,
    content: String,
}

#[derive(Clone)]
struct State {
    pool: Pool<ConnectionManager<SqliteConnection>>,
}

embed_migrations!();
#[async_std::main]
async fn main() -> tide::Result<()> {
    ctrlc::set_handler(move || {
        println!("Received Signal Ctrl+c");
        exit(0);
    })
    .expect("Error setting Ctrl-C handler");
    let (pool, state) = set_up_connection_pool_and_state();

    let migration_result = embedded_migrations::run(&pool.clone().get().unwrap());
    migration_result.unwrap_or_else(|_| panic!("Error migrating database"));
    let mut app = tide::with_state(state);
    app.at("/todo").post(add_todo);
    app.at("/todo/:uuid").get(get_todo);
    app.at("/todos").get(get_all_todos);
    // Missing error handling if anything fails during the processing
    dotenv().ok();
    // Restrict this to something realistic
    let cors = CorsMiddleware::new()
        .allow_methods("GET,POST,OPTIONS".parse::<HeaderValue>().unwrap())
        .allow_origin(Origin::from("*"))
        .allow_credentials(false);
    app.with(cors);
    app.listen(
        env::var("HOST").expect("HOST must be set")
            + ":"
            + env::var("PORT").expect("PORT must be set").as_str(),
    )
    .await?;
    Ok(())
}

fn set_up_connection_pool_and_state() -> (Pool<ConnectionManager<SqliteConnection>>, State) {
    dotenv().ok();

    let manager =
        ConnectionManager::new(env::var("DATABASE_URL").expect("DATABASE_URL must be set"));
    let pool: Pool<ConnectionManager<SqliteConnection>> =
        Pool::builder().max_size(15).build(manager).unwrap();

    let state = State { pool: pool.clone() };
    (pool, state)
}

async fn get_all_todos(req: Request<State>) -> tide::Result {
    let conn = get_connection_from_state(req);
    let results = todos.load::<Todo>(&conn).expect("Error loading posts");

    let mut todo_list = Todolist { todos: vec![] };

    for todo in results {
        todo_list.todos.push(todo);
    }

    let mut response = Response::new(200);
    response.set_body(Body::from_json(&todo_list)?);
    Ok(response)
}

fn get_connection_from_state(
    req: Request<State>,
) -> PooledConnection<ConnectionManager<SqliteConnection>> {
    req.state().pool.get().unwrap()
}

fn print_and_get_error_message(error_message: String) -> tide::Result {
    println!(
        "Processing of request finished with error >{}<",
        &error_message
    );
    Ok(Response::from(Error::from_str(400, error_message)).into())
}

async fn get_todo(req: Request<State>) -> tide::Result {
    let todo_uuid = match req.param("uuid") {
        Ok(uuid_str) => match Uuid::parse_str(uuid_str) {
            Ok(uuid) => uuid,
            Err(_) => return print_and_get_error_message("uuid could not be parsed".to_owned()),
        },
        Err(_) => return print_and_get_error_message("missing uuid path variable".to_owned()),
    };

    let conn = get_connection_from_state(req);
    let found_todos = todos
        .filter(todo_in_rust_with_tide::schema::todos::id.eq(todo_uuid.to_string()))
        .limit(1)
        .load::<Todo>(&conn)?;
    let todo = found_todos.first();
    Ok(format!(
        "You need to finish {}. If you don't remember, it is about {}\n",
        todo.unwrap().title.as_ref().unwrap(),
        todo.unwrap().content.as_ref().unwrap()
    )
    .into())
}

async fn add_todo(mut req: Request<State>) -> tide::Result {
    use todo_in_rust_with_tide::schema::todos;
    let NewRequestTodo { title, content } = match req.body_json().await {
        Ok(result) => result,
        Err(error) => return print_and_get_error_message(error.to_string()),
    };
    let conn = get_connection_from_state(req);

    let new_uuid_for_todo = Uuid::new_v4();
    let new_todo = Todo {
        id: new_uuid_for_todo.to_string(),
        title: Option::from(title),
        content: Option::from(content),
    };

    match diesel::insert_into(todos::table)
        .values(&new_todo)
        .execute(&conn)
    {
        Ok(result) => println!("{} todo(s) inserted successfully", result),
        Err(err) => return print_and_get_error_message(err.to_string()),
    };
    return Ok(format!("{}\n", new_uuid_for_todo).into());
}
