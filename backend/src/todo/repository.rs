use crate::{diesel::RunQueryDsl, todo::entities::Todo};
use diesel::{insert_into, prelude::*, result::Error, SqliteConnection};

pub struct TodoRepositoryFactory;
impl TodoRepositoryFactory {
    pub fn make(connection: &SqliteConnection) -> TodoRepository {
        TodoRepository::new(&connection)
    }
}

pub struct TodoRepository<'a> {
    connection: &'a SqliteConnection,
}
impl TodoRepository<'_> {
    fn new(connection: &SqliteConnection) -> TodoRepository {
        TodoRepository { connection }
    }

    pub fn insert(&self) -> Result<usize, Error> {
        use crate::schema::todos::dsl::*;
        insert_into(todos)
            .values((
                id.eq(1),
                title.eq("My first todo"),
                description.eq("This is my first todo"),
                is_done.eq(false),
            ))
            .execute(self.connection)
    }

    pub fn select_all(&self) -> Result<Vec<Todo>, Error> {
        use crate::schema::todos::dsl::*;
        todos.load::<Todo>(self.connection)
    }
}
