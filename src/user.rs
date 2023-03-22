use serde::Deserialize;
use utoipa::ToSchema;
#[derive(Deserialize, ToSchema)]
pub struct User {
    pub name: String,
}
impl From<String> for User {
    fn from(value: String) -> Self {
        User { name: value }
    }
}
