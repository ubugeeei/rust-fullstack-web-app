#![allow(clippy::all, warnings)]
pub struct IncompleteTodoMutation;
pub mod incomplete_todo_mutation {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "IncompleteTodoMutation";
    pub const QUERY: &str =
        "mutation IncompleteTodoMutation($id: Int!) {\n\tincompleteTodo(id: $id)\n}\n";
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
        pub id: Int,
    }
    impl Variables {}
    #[derive(Deserialize)]
    pub struct ResponseData {
        #[serde(rename = "incompleteTodo")]
        pub incomplete_todo: Boolean,
    }
}
impl graphql_client::GraphQLQuery for IncompleteTodoMutation {
    type Variables = incomplete_todo_mutation::Variables;
    type ResponseData = incomplete_todo_mutation::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: incomplete_todo_mutation::QUERY,
            operation_name: incomplete_todo_mutation::OPERATION_NAME,
        }
    }
}
