use crate::database::data;
use crate::user;
use crate::userdata;
use actix_web::{get, post, web, HttpResponse};
use fast_qr::{convert::image::ImageBuilder, qr::QRBuilder};
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

    #[utoipa::path(post,path="/api/userinfo/{name}",params(("name",description="Name of the User")),request_body(content=[UserData],example = json!([{"Contact":{"mobile_number":9876543210i64,"address":"This is my address"}},{"Instagram":{"link":"https://instagram.com"}},{"Github":{"link":"https://github.com"}}])))]
    #[post("{name}")]
    async fn push_info(
        name: web::Path<user::User>,
        pool: web::Data<PgPool>,
        data: web::Json<Vec<userdata::UserData>>,
    ) -> Result<HttpResponse, Box<dyn Error>> {
        let user = data::get_user(&name, &pool).await?;
        let _ = data::insert_multiple_user_data(&user, &data, &pool).await?;
        Ok(HttpResponse::Ok().body("Inserted multiple"))
    }
    #[utoipa::path(get,path="/api/userinfo/{name}",responses((status=200,description="went good",body=[Vec<UserData>])),params(("name",description="Name of the User")))]
    #[get("{name}")]
    async fn get_info(
        name: web::Path<user::User>,
        pool: web::Data<PgPool>,
    ) -> Result<HttpResponse, Box<dyn Error>> {
        let user = data::get_user(&name, &pool).await?;
        let data = data::get_info(&user, &pool).await?;
        Ok(HttpResponse::Ok().json(data))
    }
}

pub fn api_config(x: &mut web::ServiceConfig) {
    x.service(
        web::scope("api")
            .service(api::api_index_url)
            .service(
                web::scope("user")
                    .service(api::user_id)
                    .service(api::push_user),
            )
            .service(
                web::scope("userinfo")
                    .service(api::push_info)
                    .service(api::get_info),
            ),
    );
}

mod qr {
    use super::*;
    use actix_web::http::header;
    #[get("{name}")]
    async fn get_qr(name: web::Path<user::User>) -> Result<HttpResponse, Box<dyn Error>> {
        let userinfo_url = format!("http://localhost:8080/userinfo/{}", name.name);
        let qr = QRBuilder::new(userinfo_url)
            .build()
            .map_err(|err| format!("{:?}", err))?;
        let image = ImageBuilder::default()
            .fit_height(200)
            .fit_width(200)
            .to_pixmap(&qr);
        let image = image.encode_png()?;
        Ok(HttpResponse::Ok()
            .content_type(header::ContentType::png())
            .body(image))
    }
}

pub fn qr_config(x: &mut web::ServiceConfig) {
    x.service(web::scope("qr").service(qr::get_qr));
}
