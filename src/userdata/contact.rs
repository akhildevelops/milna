use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Deserialize, ToSchema)]
pub struct Contact {
    pub mobile_number: u64,
    pub address: String,
}
