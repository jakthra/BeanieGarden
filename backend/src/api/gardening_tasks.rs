use leptos::prelude::*;

#[server]
pub async fn get_gardening_tasks() -> Result<(), ServerFnError> {
    println!("Running query");
    use sqlx::postgres::PgPool;
    let pool =
        PgPool::connect("postgresql://postgres:postgres@localhost:5432/beaniegarden").await?;
    let rows = sqlx::query_as("select * from public.gardening_task")
        .fetch_one(&pool)
        .await?;

    Ok(rows)
}
