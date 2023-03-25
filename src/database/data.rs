use crate::database::models;
use crate::user::User;
use crate::userdata::{Contact, Facebook, Github, Instagram, UserData};
use anyhow::Result;
use futures::future;
use sqlx::postgres::PgRow;
use sqlx::{self, postgres::PgPool, sqlx_macros};
pub(crate) async fn insert_user(user: &User, pool: &PgPool) -> Result<models::user> {
    let row = sqlx::query_as::<_, models::user>(&format!(
        "INSERT INTO users (name) VALUES ({}) RETURNING id, created_at, name",
        user.name
    ))
    .fetch_one(pool)
    .await?;
    Ok(row)
}
pub(crate) async fn get_user(user: &User, pool: &PgPool) -> Result<models::user> {
    Ok(
        sqlx::query_as::<_, models::user>(&format!("SELECT * FROM users WHERE name={}", user.name))
            .fetch_one(pool)
            .await?,
    )
}

async fn insert_instagram(instagram: &Instagram, pool: &PgPool) -> Result<models::instagram> {
    let row = sqlx::query_as::<_, models::instagram>(&format!(
        "INSERT INTO instagram (link) VALUES ({}) RETURNING id, created_at, link",
        instagram.link
    ))
    .fetch_one(pool)
    .await?;
    Ok(row)
}

async fn insert_contact(contact: &Contact, pool: &PgPool) -> Result<models::contact> {
    let row = sqlx::query_as::<_,models::contact>(&format!("INSERT INTO contact (mobile_number,address) VALUES ({:?},{:?}) RETURNING id, created_at, mobile_number, address",contact.mobile_number,contact.address))
    .fetch_one(pool)
    .await?;
    Ok(row)
}
async fn insert_facebook(facebook: &Facebook, pool: &PgPool) -> Result<models::facebook> {
    let row = sqlx::query_as::<_, models::facebook>(&format!(
        "INSERT INTO facebook (link) VALUES ({}) RETURNING id, created_at, link",
        facebook.link
    ))
    .fetch_one(pool)
    .await?;
    Ok(row)
}
async fn insert_github(github: &Github, pool: &PgPool) -> Result<models::github> {
    let row = sqlx::query_as::<_, models::github>(&format!(
        "INSERT INTO github (link) VALUES ({}) RETURNING id, created_at, link",
        github.link
    ))
    .fetch_one(pool)
    .await?;
    Ok(row)
}

async fn get_github(x: &i32, pool: &PgPool) -> Result<models::github> {
    let query = format!(
        "SELECT * FROM github WHERE id={} ORDER BY created_at DESC;",
        x
    );
    Ok(sqlx::query_as::<_, models::github>(&query)
        .fetch_one(pool)
        .await?)
}

async fn get_contact(x: &i32, pool: &PgPool) -> Result<models::contact> {
    let query = format!(
        "SELECT * FROM contact WHERE id={} ORDER BY created_at DESC;",
        x
    );
    Ok(sqlx::query_as::<_, models::contact>(&query)
        .fetch_one(pool)
        .await?)
}
async fn get_instagram(x: &i32, pool: &PgPool) -> Result<models::instagram> {
    let query = format!(
        "SELECT * FROM instagram WHERE id={} ORDER BY created_at DESC;",
        x
    );
    Ok(sqlx::query_as::<_, models::instagram>(&query)
        .fetch_one(pool)
        .await?)
}
async fn get_facebook(x: &i32, pool: &PgPool) -> Result<models::facebook> {
    let query = format!(
        "SELECT * FROM facebook WHERE id={} ORDER BY created_at DESC;",
        x
    );
    Ok(sqlx::query_as::<_, models::facebook>(&query)
        .fetch_one(pool)
        .await?)
}
// async fn insert_github(github: &Github, pool: &PgPool) -> Result<models::github> {
//     let insert_query =
//         format!("INSERT INTO github (link) VALUES ($1) RETURNING id, created_at, link");

//     Ok(sqlx::query_as::<_, models::github>(&insert_query)
//         .bind(&github.link)
//         .fetch_one(pool)
//         .await?)
// }

fn generate_insert_str(user_id: &i32, data_id: &i32, data_str: &str) -> String {
    format!(
        "INSERT INTO data (user_id,{}) VALUES ({},{}) RETURNING id, created_at, instagram_id, github_id, facebook_id, contact_id, user_id",
        data_str, user_id, data_id
    )
}

async fn insert_data(data: &UserData, pool: &PgPool) -> Result<i32> {
    let id = match data {
        UserData::Contact(x) => insert_contact(x, pool).await?.id,
        UserData::Facebook(x) => insert_facebook(x, pool).await?.id,
        UserData::Github(x) => insert_github(x, pool).await?.id,
        UserData::Instagram(x) => insert_instagram(x, pool).await?.id,
    };
    // let insert_str = generate_insert_str(&user.id, &data_id_type.0, &data_id_type.1);
    // println!("{}", insert_str);
    // let data: models::data = sqlx::query_as(&insert_str).fetch_one(pool).await?;
    Ok(id)
}

pub(crate) async fn insert_multiple_user_data(
    user: &models::user,
    data: &[UserData],
    pool: &PgPool,
) -> Result<i32> {
    // future::join_all(data.iter().map(|x| insert_user_data(user, x, pool))).await
    let data_ids = future::join_all(data.iter().map(|x| insert_data(x, pool))).await;
    // use sqlx::QueryBuilder;
    // let mut qb = QueryBuilder::new("INSERT INTO data(user_id");
    // let mut sep = qb.separated(",");
    let mut insert_statement = "INSERT INTO data (user_id".to_string();
    for ud in data {
        insert_statement.push_str(&format!(",{}", ud.to_string()));
    }
    insert_statement.push_str(&format!(") VALUES ({}", user.id));
    for id in data_ids {
        insert_statement.push_str(&format!(",{}", id?));
    }
    insert_statement.push_str(") RETURNING * ;");
    Ok(sqlx::query_as::<_, models::data>(&insert_statement)
        .fetch_one(pool)
        .await?
        .id)
}

pub(crate) async fn get_info(user: &models::user, pool: &PgPool) -> Result<Vec<UserData>> {
    let mut userdata = vec![];
    let links_ids_query = format!(
        "SELECT * from data where user_id={} ORDER BY created_at DESC;",
        user.id
    );
    let result = sqlx::query_as::<_, models::data>(&links_ids_query)
        .fetch_one(pool)
        .await?;
    if let Some(x) = result.contact_id {
        let res = get_contact(&x, pool).await?;
        userdata.push(UserData::Contact(res.into()));
    };
    if let Some(x) = result.facebook_id {
        let res = get_facebook(&x, pool).await?;
        userdata.push(UserData::Facebook(res.into()));
    };
    if let Some(x) = result.github_id {
        let res = get_github(&x, pool).await?;
        userdata.push(UserData::Github(res.into()));
    };
    if let Some(x) = result.instagram_id {
        let res = get_instagram(&x, pool).await?;
        userdata.push(UserData::Instagram(res.into()));
    };
    Ok(userdata)
}

#[cfg(test)]
mod test {
    use super::*;

    #[sqlx_macros::test]
    fn user_insert() {
        let pool = sqlx::postgres::PgPool::connect("postgres://postgres:postgres@db:5432/milna")
            .await
            .unwrap();
        let user = User {
            name: "Akhil".to_string(),
        };
        insert_user(&user, &pool).await.unwrap();
    }

    #[sqlx_macros::test]
    fn facebook_insert() {
        let pool = sqlx::postgres::PgPool::connect("postgres://postgres:postgres@db:5432/milna")
            .await
            .unwrap();
        let fb = Facebook {
            link: "https://facebook.com/akhilprofile".to_string(),
        };
        insert_facebook(&fb, &pool).await.unwrap();
    }

    #[sqlx_macros::test]
    fn data_insert() {
        let pool = sqlx::postgres::PgPool::connect("postgres://postgres:postgres@db:5432/milna")
            .await
            .unwrap();
        let fb = Facebook {
            link: "https://facebook.com/akhil2ndprofile".to_string(),
        };
        let user = User {
            name: "Akhil".to_string(),
        };
        let userdata = UserData::Facebook(fb);
        let user = get_user(&user, &pool).await.unwrap();
        insert_multiple_user_data(&user, &[userdata], &pool)
            .await
            .unwrap();
    }

    #[sqlx_macros::test]
    fn data_insert_multiple() {
        let pool = sqlx::postgres::PgPool::connect("postgres://postgres:postgres@db:5432/milna")
            .await
            .unwrap();
        let fb = Facebook {
            link: "https://facebook.com/akhil2ndprofile".to_string(),
        };
        let gb = Github {
            link: "https://github.com/akhil2ndprofile".to_string(),
        };
        let user = User {
            name: "Akhil".to_string(),
        };
        let userdata = [UserData::Facebook(fb), UserData::Github(gb)];
        let user = get_user(&user, &pool).await.unwrap();
        insert_multiple_user_data(&user, &userdata, &pool)
            .await
            .unwrap();
    }
    #[sqlx_macros::test]
    fn get_links() {
        let pool = sqlx::postgres::PgPool::connect("postgres://postgres:postgres@db:5432/milna")
            .await
            .unwrap();
        let user = User {
            name: "Akhil".to_string(),
        };
        let user = get_user(&user, &pool).await.unwrap();
        get_info(&user, &pool).await.unwrap(); // TODO: No assert to validate
    }
}
