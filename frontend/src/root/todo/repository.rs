use graphql_client::{GraphQLQuery, Response};
use reqwest;
use std::error::Error;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/schema.graphql",
    query_path = "src/graphql/todo/get_todos.graphql",
    response_derives = "Serialize,PartialEq,Clone"
)]
pub struct GetTodosQuery;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/schema.graphql",
    query_path = "src/graphql/todo/create_todo.graphql",
    response_derives = "Serialize,PartialEq,Clone"
)]
pub struct CreateTodoMutation;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/schema.graphql",
    query_path = "src/graphql/todo/complete_todo.graphql",
    response_derives = "Serialize,PartialEq,Clone"
)]
pub struct CompleteTodoMutation;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/schema.graphql",
    query_path = "src/graphql/todo/incomplete_todo.graphql",
    response_derives = "Serialize,PartialEq,Clone"
)]
pub struct IncompleteTodoMutation;

pub struct TodoRepository;
impl TodoRepository {
    pub async fn get(
        variables: get_todos_query::Variables,
    ) -> Result<get_todos_query::ResponseData, Box<dyn Error>> {
        let request_body = GetTodosQuery::build_query(variables);

        let client = reqwest::Client::new();
        let res = client
            .post("http://127.0.0.1:4000")
            .json(&request_body)
            .send()
            .await?;
        let response_body: Response<get_todos_query::ResponseData> = res.json().await?;

        Ok(response_body.data.unwrap())
    }

    pub async fn create(variables: create_todo_mutation::Variables) -> Result<(), Box<dyn Error>> {
        let request_body = CreateTodoMutation::build_query(variables);

        let client = reqwest::Client::new();
        client
            .post("http://127.0.0.1:4000")
            .json(&request_body)
            .send()
            .await?;

        Ok(())
    }

    pub async fn complete(
        variables: complete_todo_mutation::Variables,
    ) -> Result<(), Box<dyn Error>> {
        let request_body = CompleteTodoMutation::build_query(variables);

        let client = reqwest::Client::new();
        client
            .post("http://127.0.0.1:4000")
            .json(&request_body)
            .send()
            .await?;
        Ok(())
    }

    pub async fn incomplete(
        variables: incomplete_todo_mutation::Variables,
    ) -> Result<(), Box<dyn Error>> {
        let request_body = IncompleteTodoMutation::build_query(variables);

        let client = reqwest::Client::new();
        client
            .post("http://127.0.0.1:4000")
            .json(&request_body)
            .send()
            .await?;
        Ok(())
    }
}
