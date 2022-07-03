use super::{
    entities::Todo, factory::TodoFactory, repository::TodoRepository, service::TodoService,
};
use crate::{establish_connection, Mutation, Query};
use async_graphql::Object;

#[Object]
impl Query {
    async fn get_todos(&self) -> Vec<Todo> {
        // TODO: share this connection with the service
        let todo_service = TodoService::new(
            establish_connection(),
            TodoRepository::new(),
            TodoFactory::new(),
        );
        todo_service.get_todo()
    }
}

#[Object]
impl Mutation {
    async fn create_todo(&self, title: String, description: String) -> bool {
        // TODO: share this connection with the service
        let todo_service = TodoService::new(
            establish_connection(),
            TodoRepository::new(),
            TodoFactory::new(),
        );
        todo_service.create_todo(title, description)
    }
}
