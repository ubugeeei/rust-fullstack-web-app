use diesel::Queryable;
use once_cell::sync::Lazy;
use std::sync::Mutex;

// TODO: unique
static SEQUENCE_ID: Lazy<Mutex<usize>> = Lazy::new(|| Mutex::new(0));

#[derive(Queryable)]
pub struct Todo {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
    pub is_done: bool,
}

impl Todo {
    fn new(title: String, description: Option<String>) -> Todo {
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