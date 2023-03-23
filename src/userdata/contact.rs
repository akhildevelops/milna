use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Deserialize, Serialize, ToSchema)]
pub struct Contact {
    pub mobile_number: Option<String>,
    pub address: Option<String>,
}
