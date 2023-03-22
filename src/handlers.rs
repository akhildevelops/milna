use crate::database::data;
use crate::user;
use actix_web::{get, post, web, HttpResponse};
use serde::Serialize;
use sqlx::postgres::PgPool;
use std::error::Error;

// TODO: In example attrs, replace them with env!()
#[derive(Serialize, utoipa::ToSchema)]
pub struct IndexResponse {
    #[schema(example = "0.1.0")]
    version: &'static str,
    #[schema(example = "milna")]
    name: &'static str,
    #[schema(example = "Connects socially")]
    description: &'static str,
}
const INDEX_URL_RESPONSE: IndexResponse = IndexResponse {
    version: env!("CARGO_PKG_VERSION"),
    name: env!("CARGO_PKG_NAME"),
    description: env!("CARGO_PKG_DESCRIPTION"),
};

#[utoipa::path(get,path="/",
    responses((status=200,description="Index URL",body=[IndexResponse]))
)]
#[get("/")]
async fn index_url() -> HttpResponse {
    HttpResponse::Ok().json(INDEX_URL_RESPONSE)
}

pub mod api {
    use super::*;

    #[get("")]
    async fn api_index_url() -> HttpResponse {
        HttpResponse::Ok().json(INDEX_URL_RESPONSE)
    }
    #[utoipa::path(get,
        path="/api/user/{name}",
        params(("name",description="Name of the User")),
        responses((status=200,description="Returns User details",body=[User]))
    )]
    #[get("{name}")]
    async fn user_id(
        x: web::Path<user::User>,
        pgpool: web::Data<PgPool>,
    ) -> Result<HttpResponse, Box<dyn Error>> {
        let user = data::get_user(&x, &pgpool).await?;
        Ok(HttpResponse::Ok().json(user))
    }
    #[utoipa::path(post, path = "/api/user",request_body=User)]
    #[post("")] // TODO: Correct this "" looks akward
    async fn push_user(
        x: web::Json<user::User>,
        pgpool: web::Data<PgPool>,
    ) -> Result<HttpResponse, Box<dyn Error>> {
        let user = data::insert_user(&x, &pgpool).await?;
        Ok(HttpResponse::Ok().json(user))
    }
}

pub fn api_config(x: &mut web::ServiceConfig) {
    x.service(
        web::scope("api").service(api::api_index_url).service(
            web::scope("user")
                .service(api::user_id)
                .service(api::push_user),
        ),
    );
}
