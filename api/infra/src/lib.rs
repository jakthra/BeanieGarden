use sqlx::postgres::PgPool;

pub fn get_dsn() -> String {
    std::env::var("DATABASE_URL").unwrap_or_else(|_| {
        "postgresql://postgres:postgres@localhost:5432/beaniegarden".to_string()
    })
}

pub async fn get_db_pool() -> PgPool {
    PgPool::connect(&get_dsn()).await.unwrap()
}
