use serde::Deserialize;
#[derive(Deserialize)]
pub(crate) struct User {
    pub name: String,
}
impl From<String> for User {
    fn from(value: String) -> Self {
        User { name: value }
    }
}
