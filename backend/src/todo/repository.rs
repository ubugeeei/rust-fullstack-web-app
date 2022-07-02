use crate::{diesel::RunQueryDsl, todo::entities::Todo};
use diesel::{insert_into, prelude::*, result::Error, SqliteConnection};

pub struct TodoRepository;

impl TodoRepository {
    pub fn new() -> Self {
        TodoRepository
    }

    pub fn insert(
        &self,
        connection: &SqliteConnection,
        _title: &str,
        _description: &str,
    ) -> Result<usize, Error> {
        use crate::schema::todos::dsl::*;
        insert_into(todos)
            .values((
                id.eq(1),
                title.eq(_title),
                description.eq(_description),
                is_done.eq(false),
            ))
            .execute(connection)
    }

    pub fn select_all(&self, connection: &SqliteConnection) -> Result<Vec<Todo>, Error> {
        use crate::schema::todos::dsl::*;
        todos.load::<Todo>(connection)
    }
}
