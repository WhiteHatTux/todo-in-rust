use tide::prelude::*;
use tide::Request;

#[derive(Debug, Deserialize)]
struct Animal {
    title: String,
    content: String,
    present: bool,
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    let mut app = tide::new();
    app.at("/todo").post(add_todo);
    app.listen("127.0.0.1:8083").await?;
    Ok(())
}

async fn add_todo(mut req: Request<()>) -> tide::Result {
    let Animal { title, content, present } = req.body_json().await?;
    if present {
        Ok(format!("I've put in a new todo {} with content {}. You need to present it to your dad", title, content).into())
    } else {
        Ok(format!("I've put in a new todo {} with content {}", title, content).into())
    }
}
