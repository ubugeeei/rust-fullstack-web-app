use async_graphql::SimpleObject;
use diesel::Queryable;

#[derive(Queryable, Clone)]
pub struct ORMTodo {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub is_done: bool,
}

#[derive(SimpleObject, Clone)]
pub struct Todo {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub is_done: bool,
}
