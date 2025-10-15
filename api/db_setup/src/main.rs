

use std::{fs, path::Path};

use infra::postgres::get_dsn;
use sea_orm::{Database, DatabaseConnection, ExecResult};
use sea_orm::ConnectionTrait;
use seeder::seed;


fn get_search_index_query() -> Result<String, std::io::Error> {
    let sql_dir = "sql";
    let sql_index_file = "search_index.sql";
    let path = Path::new(&sql_dir).join(sql_index_file);
    println!("{:?}", path);
    match fs::read_to_string(&path) {
        Ok(sql) => return Ok(sql),
        Err(e) => return Err(e)
    }

}

pub async fn search_index() -> Result<ExecResult, sea_orm::DbErr> {
    let db: DatabaseConnection = Database::connect(get_dsn()).await.unwrap();
    let search_index_query = get_search_index_query().unwrap();
    db.execute(sea_orm::Statement { sql: search_index_query, values: None, db_backend: sea_orm::DatabaseBackend::Postgres }).await
}


#[tokio::main]
async fn main() {
    // let _ = seed().await;
    let _ = search_index().await;
}
