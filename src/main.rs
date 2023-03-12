use actix_web::{web, App, HttpServer, Responder};
use milna::handlers;
#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    HttpServer::new(|| App::new().service(handlers::index_url))
        .bind(("localhost", 8080))?
        .run()
        .await
}
