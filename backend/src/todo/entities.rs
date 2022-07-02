use async_graphql::SimpleObject;
use diesel::Queryable;
use once_cell::sync::Lazy;
use std::sync::Mutex;

// TODO: unique
#[allow(dead_code)]
static SEQUENCE_ID: Lazy<Mutex<i32>> = Lazy::new(|| Mutex::new(0));

#[derive(Queryable, SimpleObject, Clone)]
pub struct Todo {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub is_done: bool,
}

impl Todo {
    #[allow(dead_code)]
    fn new(title: String, description: String) -> Todo {
        let mut id = SEQUENCE_ID.lock().unwrap();
        *id += 1;
        Todo {
            id: *id,
            title,
            description,
            is_done: false,
        }
    }
}
