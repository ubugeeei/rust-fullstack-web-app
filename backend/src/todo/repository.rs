use crate::{diesel::RunQueryDsl, todo::entities::ORMTodo};
use diesel::{insert_into, prelude::*, result::Error, SqliteConnection};
use nanoid::nanoid;

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
        let _id = nanoid!(9, &['1', '2', '3', '4', '5', '6', '7', '8', '9'])
            .parse::<i32>()
            .unwrap();

        insert_into(todos)
            .values((
                id.eq(_id),
                title.eq(_title),
                description.eq(_description),
                is_done.eq(false),
            ))
            .execute(connection)
    }

    pub fn select_all(&self, connection: &SqliteConnection) -> Result<Vec<ORMTodo>, Error> {
        use crate::schema::todos::dsl::*;
        todos.load::<ORMTodo>(connection)
    }
}
