#![allow(clippy::all, warnings)]
pub struct CreateTodoMutation;
pub mod create_todo_mutation {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "CreateTodoMutation";
    pub const QUERY : & str = "mutation CreateTodoMutation($title: String!, $description: String!) {\n\tcreateTodo(title: $title, description: $description)\n}\n" ;
    use super::*;
    use serde::{Deserialize, Serialize};
    #[allow(dead_code)]
    type Boolean = bool;
    #[allow(dead_code)]
    type Float = f64;
    #[allow(dead_code)]
    type Int = i64;
    #[allow(dead_code)]
    type ID = String;
    #[derive(Serialize)]
    pub struct Variables {
        pub title: String,
        pub description: String,
    }
    impl Variables {}
    #[derive(Deserialize)]
    pub struct ResponseData {
        #[serde(rename = "createTodo")]
        pub create_todo: Boolean,
    }
}
impl graphql_client::GraphQLQuery for CreateTodoMutation {
    type Variables = create_todo_mutation::Variables;
    type ResponseData = create_todo_mutation::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: create_todo_mutation::QUERY,
            operation_name: create_todo_mutation::OPERATION_NAME,
        }
    }
}
