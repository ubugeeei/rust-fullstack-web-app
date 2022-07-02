use std::env;

use diesel::{Connection, SqliteConnection};
use dotenv::dotenv;
use todo::repository::TodoRepository;

#[macro_use]
extern crate diesel;

pub mod schema;

pub mod todo;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not defined!");

    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

fn main() {
    let connection = establish_connection();
    let todo_repo = TodoRepository::new();

    let insert_res = todo_repo.insert(&connection, "new todo", "some description");

    match insert_res {
        Ok(_) => println!("Inserted a new todo!"),
        Err(e) => println!("Error inserting a new todo: {:?}", e),
    }

    let res = todo_repo.select_all(&connection);
    match res {
        Ok(todos) => {
            for todo in todos {
                println!("id: {}", todo.id);
                println!("title: {}", todo.title);
                println!("description: {}", todo.description);
                println!("is_done: {}", todo.is_done);
            }
        }
        Err(e) => println!("Error selecting all todos: {:?}", e),
    }
}
