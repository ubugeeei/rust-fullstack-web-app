#[macro_use]
extern crate diesel;
use actix_cors::Cors;
use actix_web::{guard, web, web::Data, App, HttpResponse, HttpServer, Result};

use async_graphql::{
    http::{playground_source, GraphQLPlaygroundConfig},
    EmptySubscription, Object, Schema,
};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use diesel::{Connection, SqliteConnection};
use dotenv::dotenv;
use std::env;
pub mod schema;
pub mod todo;
use crate::todo::service::TodoService;
use todo::{entities::Todo, factory::TodoFactory, repository::TodoRepository};

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not defined!");

    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    struct Query;
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

    struct Mutation;
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

    type ApiSchema = Schema<Query, Mutation, EmptySubscription>;

    async fn index(schema: web::Data<ApiSchema>, req: GraphQLRequest) -> GraphQLResponse {
        schema.execute(req.into_inner()).await.into()
    }

    async fn index_playground() -> Result<HttpResponse> {
        let source =
            playground_source(GraphQLPlaygroundConfig::new("/").subscription_endpoint("/"));
        Ok(HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body(source))
    }

    let schema = Schema::build(Query, Mutation, EmptySubscription).finish();

    println!("listen ...");
    println!("http://127.0.0.1:4000");

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://127.0.0.1:8080")
            .allowed_methods(vec!["GET", "POST"]);

        App::new()
        .wrap(cors)
        .app_data(Data::new(schema.clone()))
            .service(web::resource("/").guard(guard::Post()).to(index))
            .service(web::resource("/").guard(guard::Get()).to(index_playground))
    })
    .bind("127.0.0.1:4000")?
    .run()
    .await
}
