use tide::prelude::Serialize;

use super::schema::todos;

#[derive(Debug, Clone, Serialize, Queryable, Insertable)]
#[table_name = "todos"]
pub struct Todo {
    pub id: String,
    pub title: Option<String>,
    pub content: Option<String>,
    pub done: bool,
}
