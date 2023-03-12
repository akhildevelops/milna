// use chrono::
use chrono::NaiveDateTime;
use serde::Serialize;
use serde::{self, Serializer};
use sqlx::FromRow;

fn nativedate_ser<T: Serializer>(ts: &NaiveDateTime, s: T) -> Result<T::Ok, T::Error> {
    s.serialize_str(&ts.format("%Y-%m-%d %H:%M:%S").to_string())
}
#[derive(FromRow, Serialize)]
#[allow(non_camel_case_types)]
pub(crate) struct user {
    pub id: i32,
    #[serde(serialize_with = "nativedate_ser")]
    pub created_at: NaiveDateTime,
    pub name: String,
}

#[derive(FromRow)]
#[allow(non_camel_case_types)]
pub(crate) struct contact {
    pub id: i32,
    pub created_at: NaiveDateTime,
    pub mobile_number: Option<i32>,
    pub address: Option<String>,
}

macro_rules! link_models {
    ($($x:ident),*) => {
        $(
            #[derive(FromRow)]
            #[allow(non_camel_case_types)]
            pub(crate) struct $x {
                pub id: i32,
                pub created_at: NaiveDateTime,
                pub link: String,
            }
        )*
    };
}

link_models!(instagram, facebook, github);

#[derive(FromRow)]
#[allow(non_camel_case_types)]
pub(crate) struct data {
    pub id: i32,
    pub created_at: NaiveDateTime,
    pub instagram_id: Option<i32>,
    pub github_id: Option<i32>,
    pub facebook_id: Option<i32>,
    pub contact_id: Option<i32>,
    pub user_id: i32,
}
