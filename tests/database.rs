use sqlx::migrate::MigrateDatabase;
use sqlx::sqlx_macros;
use std::default::Default;
use std::env;
use std::error::Error;
struct ENV<T> {
    postgres_user: T,
    postgres_password: T,
    postgres_port: T,
    postgres_host: T,
    postgres_db: T,
}

impl Default for ENV<String> {
    fn default() -> Self {
        Self {
            postgres_user: "POSTGRES_USER".to_string(),
            postgres_password: "POSTGRES_PASSWORD".to_string(),
            postgres_port: "POSTGRES_PORT".to_string(),
            postgres_host: "POSTGRES_HOSTNAME".to_string(),
            postgres_db: "POSTGRES_DB".to_string(),
        }
    }
}

struct PostgresEnv {
    postgres_user: String,
    postgres_password: String,
    postgres_port: usize,
    postgres_host: String,
    postgres_db: String,
}

impl PostgresEnv {
    fn from_env() -> Result<Self, Box<dyn Error>> {
        let env_keys = ENV::default();
        let postgres_user = env::var(&env_keys.postgres_user)?;
        let postgres_password = env::var(&env_keys.postgres_password)?;
        let postgres_host = env::var(&env_keys.postgres_host)?;
        let postgres_port = match env::var(&env_keys.postgres_port) {
            Ok(x) => Ok(x.parse::<usize>()?),
            Err(x) => Err(x.to_string()),
        }?;
        let postgres_db = env::var(&env_keys.postgres_db)?;
        Ok(Self {
            postgres_user,
            postgres_password,
            postgres_port,
            postgres_host,
            postgres_db,
        })
    }

    fn to_conn_string(&self) -> String {
        format!(
            "postgresql://{}:{}@{}:{}/{}",
            self.postgres_user,
            self.postgres_password,
            self.postgres_host,
            self.postgres_port,
            self.postgres_db
        )
    }
}

#[sqlx_macros::test]
async fn test_db() {
    let postgres = PostgresEnv::from_env().unwrap();
    let postgres_str = postgres.to_conn_string();
    assert!(sqlx::Postgres::database_exists(&postgres_str)
        .await
        .unwrap());
}
