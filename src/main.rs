use actix_web::{web, App, HttpServer, Responder};
use env_logger;
use milna::handlers;
use sqlx::postgres::PgPool;
use std::{env, error::Error};
const ENV_DATABASE_URL: &'static str = "DATABASE_URL";
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();
    let db_url = env::var(ENV_DATABASE_URL)?;
    let pool = PgPool::connect(&db_url).await?;
    Ok(HttpServer::new(move || {
        App::new()
            .service(handlers::index_url)
            .app_data(web::Data::new(pool.clone()))
            .configure(handlers::api_config)
    })
    .bind(("localhost", 8080))?
    .run()
    .await?)
}
