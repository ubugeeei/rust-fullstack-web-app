#![allow(clippy::all, warnings)]
pub struct GetTodosQuery;
pub mod get_todos_query {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "GetTodosQuery";
    pub const QUERY : & str = "query GetTodosQuery {\n\tgetTodos {\n\t\tid\n\t\ttitle\n\t\tdescription\n\t\tisDone\n\t}\n}\n\n# mutation createTodo($title: String!, $description: String!) {\n# \tcreateTodo(title: $title, description: $description) {\n# \t\t# id\n# \t\ttitle\n# \t\t# description\n# \t\t# isDone\n# \t}\n# }\n" ;
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
    pub struct Variables;
    #[derive(Deserialize)]
    pub struct ResponseData {
        #[serde(rename = "getTodos")]
        pub get_todos: Vec<GetTodosQueryGetTodos>,
    }
    #[derive(Deserialize, PartialEq)]
    pub struct GetTodosQueryGetTodos {
        pub id: Int,
        pub title: String,
        pub description: String,
        #[serde(rename = "isDone")]
        pub is_done: Boolean,
    }
}
impl graphql_client::GraphQLQuery for GetTodosQuery {
    type Variables = get_todos_query::Variables;
    type ResponseData = get_todos_query::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: get_todos_query::QUERY,
            operation_name: get_todos_query::OPERATION_NAME,
        }
    }
}
