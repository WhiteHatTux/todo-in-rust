extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

use diesel::prelude::*;
use diesel_migrations::embed_migrations;
use tide::{Body, Request, Response};
use tide::prelude::*;
use uuid::Uuid;

use todo_in_rust_with_tide::establish_connection;
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

embed_migrations!();
#[async_std::main]
async fn main() -> tide::Result<()> {
    let conn = establish_connection();
    embedded_migrations::run(&conn);
    let mut app = tide::new();
    app.at("/todo").post(add_todo);
    app.at("/todo/:uuid").get(get_todo);
    app.at("/todos").get(get_all_todos);
    // Missing error handling if anything fails during the processing
    app.listen("127.0.0.1:8083").await?;
    Ok(())
}

async fn get_all_todos(_req: Request<()>) -> tide::Result {
    let conn = establish_connection();
    let results = todos
        .load::<Todo>(&conn)
        .expect("Error loading posts");

    let mut todo_list = Todolist {
        todos: vec![],
    };

    for todo in results {
        todo_list.todos.push(todo);
    }

    let mut response = Response::new(200);
    response.set_body(Body::from_json(&todo_list)?);
    Ok(response)
}

async fn get_todo(req: Request<()>) -> tide::Result {
    let todo_uuid = Uuid::parse_str(req.param("uuid").unwrap_or("failed"))?;

    let conn = establish_connection();
    let found_todos = todos.filter(todo_in_rust_with_tide::schema::todos::id.eq(todo_uuid.to_string()))
        .limit(1)
        .load::<Todo>(&conn)?;
    let todo = found_todos.first();
    Ok(format!("You need to finish {}. If you don't remember, it is about {}\n", todo.unwrap().title.as_ref().unwrap(), todo.unwrap().content.as_ref().unwrap()).into())
}

async fn add_todo(mut req: Request<()>) -> tide::Result {
    use todo_in_rust_with_tide::schema::todos;
    let NewRequestTodo { title, content } = req.body_json().await?;
    let conn = establish_connection();

    let new_uuid_for_todo = Uuid::new_v4();
    let new_todo = Todo { id: new_uuid_for_todo.to_string(), title: Option::from(title), content: Option::from(content) };

    diesel::insert_into(todos::table)
        .values(&new_todo)
        .execute(&conn)
        .expect("Error saving new todo");
    return Ok(format!("{}\n", new_uuid_for_todo).into());
}
