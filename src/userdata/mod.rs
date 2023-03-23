mod contact;
mod facebook;
mod github;
mod instagram;
use crate::database::models;
pub(crate) use contact::Contact;
pub(crate) use facebook::Facebook;
pub(crate) use github::Github;
pub(crate) use instagram::Instagram;
use serde::Deserialize;
use std::fmt;
use utoipa::ToSchema;
#[derive(Deserialize, ToSchema)]
pub enum UserData {
    #[schema(example = "Hi")]
    Contact(Contact),
    #[schema(example = "Hi")]
    Facebook(Facebook),
    #[schema(example = "Hi")]
    Github(Github),
    #[schema(example = "Hi")]
    Instagram(Instagram),
}

// TODO: Create a trait for userdata.
impl fmt::Display for UserData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UserData::Contact(_) => write!(f, "contact_id"),
            UserData::Facebook(_) => write!(f, "facebook_id"),
            UserData::Github(_) => write!(f, "github_id"),
            UserData::Instagram(_) => write!(f, "instagram_id"),
        }
    }
}

macro_rules! convert {
    ($($x:ty,$y:ident),*) => {
        $(
            impl From<$x> for $y {
                fn from(value:$x)->Self{
                    Self {link:value.link}
                }
            }
        )*
    };
}

convert!(
    models::github,
    Github,
    models::facebook,
    Facebook,
    models::instagram,
    Instagram
);

impl From<models::contact> for Contact {
    fn from(value: models::contact) -> Self {
        Self {
            mobile_number: value.mobile_number,
            address: value.address,
        }
    }
}
