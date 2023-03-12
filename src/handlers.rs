use actix_web::{get, HttpResponse};
use serde::Serialize;
#[derive(Serialize)]
struct IndexResponse {
    version: &'static str,
    name: &'static str,
    description: &'static str,
}
const INDEX_URL_RESPONSE: IndexResponse = IndexResponse {
    version: env!("CARGO_PKG_VERSION"),
    name: env!("CARGO_PKG_NAME"),
    description: env!("CARGO_PKG_DESCRIPTION"),
};
#[get("/")]
async fn index_url() -> HttpResponse {
    HttpResponse::Ok().json(INDEX_URL_RESPONSE)
}
