use diesel::SqliteConnection;

use super::{entities::Todo, factory::TodoFactory, repository::TodoRepository};

pub struct TodoService {
    connection: SqliteConnection,
    repository: TodoRepository,
    factory: TodoFactory,
}

impl TodoService {
    pub fn new(
        connection: SqliteConnection,
        repository: TodoRepository,
        factory: TodoFactory,
    ) -> TodoService {
        TodoService {
            connection,
            repository,
            factory,
        }
    }

    pub fn get_todo(&self) -> Vec<Todo> {
        let res = self.repository.select_all(&self.connection);

        match res {
            Ok(todos) => self.factory.make_todos(todos),
            Err(e) => panic!("Error selecting all todos: {:?}", e),
        }
    }

    pub fn create_todo(&self, title: String, description: String) -> bool {
        let insert_res = self
            .repository
            .insert(&self.connection, &title, &description);
        match insert_res {
            Ok(_) => true,
            Err(e) => {
                println!("Error inserting a new todo: {:?}", e);
                false
            }
        }
    }

    pub fn complete_todo(&self, id: i32) -> bool {
        let complete_res = self.repository.complete(&self.connection, id);
        match complete_res {
            Ok(_) => true,
            Err(e) => {
                println!("Error completing a todo: {:?}", e);
                false
            }
        }
    }

    pub fn incomplete_todo(&self, id: i32) -> bool {
        let incomplete_res = self.repository.incomplete(&self.connection, id);
        match incomplete_res {
            Ok(_) => true,
            Err(e) => {
                println!("Error incomplete a todo: {:?}", e);
                false
            }
        }
    }
}
