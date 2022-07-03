use super::entities::{ORMTodo, Todo};

pub struct TodoFactory;
impl TodoFactory {
    pub fn new() -> Self {
        TodoFactory
    }

    pub fn make_todos(&self, orm_todos: Vec<ORMTodo>) -> Vec<Todo> {
        orm_todos
            .into_iter()
            .map(|orm_todo| Todo {
                id: orm_todo.id,
                title: orm_todo.title,
                description: orm_todo.description,
                is_done: orm_todo.is_done,
            })
            .collect()
    }

    pub fn make_todo(&self, orm_todo: ORMTodo) -> Todo {
        Todo {
            id: orm_todo.id,
            title: orm_todo.title,
            description: orm_todo.description,
            is_done: orm_todo.is_done,
        }
    }
}
