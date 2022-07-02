#[macro_use]
extern crate diesel;
use diesel::{Connection, SqliteConnection};

use actix_web::{guard, web, web::Data, App, HttpResponse, HttpServer, Result};
use async_graphql::{
    http::{playground_source, GraphQLPlaygroundConfig},
    EmptySubscription, Object, Schema,
};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};

use dotenv::dotenv;
use std::env;

pub mod schema;
pub mod todo;
use todo::{entities::Todo, repository::TodoRepository};

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not defined!");

    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

struct Query;
#[Object]
impl Query {
    async fn get_todos(&self) -> Vec<Todo> {
        let connection = establish_connection();
        let todo_repo = TodoRepository::new();
        let res = todo_repo.select_all(&connection);
        match res {
            Ok(todos) => todos,
            Err(e) => panic!("Error selecting all todos: {:?}", e),
        }
    }
}

struct Mutation;
#[Object]
impl Mutation {
    async fn create_todo(&self, title: String, description: String) -> bool {
        let connection = establish_connection();
        let todo_repo = TodoRepository::new();
        let insert_res = todo_repo.insert(&connection, &title, &description);
        match insert_res {
            Ok(_) => true,
            Err(e) => {
                println!("Error inserting a new todo: {:?}", e);
                false
            }
        }
    }
}

type ApiSchema = Schema<Query, Mutation, EmptySubscription>;

async fn index(schema: web::Data<ApiSchema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}
async fn index_playground() -> Result<HttpResponse> {
    let source = playground_source(GraphQLPlaygroundConfig::new("/").subscription_endpoint("/"));
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(source))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let schema = Schema::build(Query, Mutation, EmptySubscription).finish();

    println!("listen ...");
    println!("http://127.0.0.1:8080");

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(schema.clone()))
            .service(web::resource("/").guard(guard::Post()).to(index))
            .service(web::resource("/").guard(guard::Get()).to(index_playground))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
