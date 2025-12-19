use sqlx::{postgres::{PgConnectOptions, PgPool}, ConnectOptions};
use url::Url;

pub fn get_dsn() -> String {
    std::env::var("DATABASE_URL").unwrap_or_else(|_| {
        "postgresql://postgres:postgres@localhost:5432/beaniegarden".to_string()
    })
}

pub fn get_db_options() -> PgConnectOptions {
    let url = Url::parse(&get_dsn()).unwrap();
    PgConnectOptions::from_url(&url).unwrap()
}


pub async fn get_db_pool() -> PgPool {
    PgPool::connect(&get_dsn()).await.unwrap()
}
