use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Deserialize, ToSchema)]
pub struct Contact {
    pub mobile_number: Option<String>,
    pub address: Option<String>,
}
