use actix_cors::Cors;
use actix_files::Files;
use actix_web::{web, App, HttpServer};
use env_logger;
use milna::handlers;
use sqlx::postgres::PgPool;
use std::{env, error::Error};
use utoipa::OpenApi;
const ENV_DATABASE_URL: &'static str = "DATABASE_URL";
#[derive(utoipa::OpenApi)]
#[openapi(
    paths(
        handlers::index_url,
        handlers::api::user_id,
        handlers::api::push_user,
        handlers::api::push_info,
        handlers::api::get_info
    ),
    components(schemas(handlers::IndexResponse, milna::user::User, milna::userdata::UserData))
)]
struct APIDOC;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let openapi = APIDOC::openapi();
    let openapi_service = utoipa_swagger_ui::SwaggerUi::new("/swagger-ui/{_:.*}")
        .url("/api-doc/openapi.json", openapi);
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();
    let db_url = env::var(ENV_DATABASE_URL)?;
    let pool = PgPool::connect(&db_url).await?;
    Ok(HttpServer::new(move || {
        let cors = Cors::default().allow_any_origin();
        App::new()
            .wrap(cors)
            .service(openapi_service.clone())
            .configure(handlers::api_config)
            .configure(handlers::qr_config)
            .service(
                Files::new("/", "svelte/build")
                    .index_file("index.html")
                    .default_handler(
                        actix_files::NamedFile::open(format!("{}/index.html", "svelte/build"))
                            .unwrap(),
                    ),
            )
            .app_data(web::Data::new(pool.clone()))
    })
    .bind(("localhost", 8080))?
    .run()
    .await?)
}
