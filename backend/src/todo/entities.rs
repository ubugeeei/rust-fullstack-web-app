use async_graphql::SimpleObject;
use diesel::Queryable;
use nanoid::nanoid;

#[derive(Queryable, SimpleObject, Clone)]
pub struct Todo {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub is_done: bool,
}
