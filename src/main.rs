use rusqlite::{Connection, params};
use tide::prelude::*;
use tide::Request;
use uuid::Uuid;

mod migrations;

#[derive(Debug, Deserialize)]
struct NewTodo {
    title: String,
    content: String,
    present: bool,
}

#[derive(Debug)]
struct Todo {
    id: Uuid,
    title: String,
    content: String,
}

#[derive(Deserialize)]
struct QueryParameters {
    uuid: Uuid,
}

#[derive(Debug)]
struct Todolist {
    todos: Vec<Todo>,
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    // Use r2d2 with diesel for database and connection pooling
    let mut conn = Connection::open("test.db")?;
    migrations::runner().run(&mut conn).unwrap();
    let mut app = tide::new();
    app.at("/todo").post(add_todo);
    app.at("/todo/{uuid}").get(get_todo);
    app.at("/todos").get(get_all_todos);
    // Missing error handling if anything fails during the processing
    app.listen("127.0.0.1:8083").await?;
    Ok(())
}

async fn get_all_todos(_req: Request<()>) -> tide::Result {
    let conn = Connection::open("test.db")?;
    let mut stmt = conn.prepare("select id, title, content from todo")?;
    let existing_todos = stmt.query_map(params![], |row| {
        Ok(Todo {
            id: row.get(0)?,
            title: row.get(1)?,
            content: row.get(2)?,
        })
    })?;
    let mut complete_string = "".to_owned();
    existing_todos.for_each(|todo| {
        complete_string.push_str(todo.unwrap().id.to_string().as_ref());
    });
    Ok(format!("found bunch of ids {}", complete_string).into())
}

async fn get_todo(req: Request<()>) -> tide::Result {
    let todo_uuid_real: QueryParameters = req.query()?;

    let todo_uuid = todo_uuid_real.uuid;
    let conn = Connection::open("test.db")?;
    let mut stmt = conn.prepare("select id, title, content from todo where id = (?1)")?;
    let mut found_todos = stmt.query_map(params![todo_uuid], |row| {
        Ok(Todo {
            id: row.get(0)?,
            title: row.get(1)?,
            content: row.get(2)?,
        })
    })?;
    let found_todo = found_todos.next();
    let todo = found_todo.unwrap()?;
    Ok(format!("You need to finish {}. If you don't remember, it is about {}\n", todo.title, todo.content).into())
}

async fn add_todo(mut req: Request<()>) -> tide::Result {
    let NewTodo { title, content, present: _ } = req.body_json().await?;
    let conn = Connection::open("test.db")?;
    let new_uuid_for_todo = Uuid::new_v4();
    conn.execute(
        "INSERT INTO todo (id, title, content) VALUES (?1, ?2, ?3)",
        params![new_uuid_for_todo, title, content],
    )?;
    return Ok(format!("{}\n", new_uuid_for_todo).into());
}
