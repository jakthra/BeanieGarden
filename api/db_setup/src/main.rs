

use std::{fs, path::Path};
use log::{info};
use infra::postgres::{get_db_options, get_dsn};
use sea_orm::{Database, DatabaseConnection, ExecResult};
use sea_orm::ConnectionTrait;
use sqlx::ConnectOptions; 
use migration::{Migrator, MigratorTrait};

pub mod seeder;
pub mod gbif_service;

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

fn get_pg_trgm_extension_query() -> Result<String, std::io::Error> {
    let sql_dir = "sql";
    let sql_index_file = "pg_trgm.sql";
    let path = Path::new(&sql_dir).join(sql_index_file);
    println!("{:?}", path);
    match fs::read_to_string(&path) {
        Ok(sql) => return Ok(sql),
        Err(e) => return Err(e)
    }

}

pub async fn pg_trgm_extension()-> Result<ExecResult, sea_orm::DbErr> {
    let db: DatabaseConnection = Database::connect(get_dsn()).await.unwrap();
    let pg_trgm_extension = get_pg_trgm_extension_query().unwrap();
    match db.execute(sea_orm::Statement { sql: pg_trgm_extension, values: None, db_backend: sea_orm::DatabaseBackend::Postgres }).await {
        Ok(result) => return Ok(result),
        Err(e) => panic!("{}", e.to_string())
    }
}

pub async fn search_index() -> Result<ExecResult, sea_orm::DbErr> {
    let db: DatabaseConnection = Database::connect(get_dsn()).await.unwrap();
    let search_index_query = get_search_index_query().unwrap();
    match db.execute(sea_orm::Statement { sql: search_index_query, values: None, db_backend: sea_orm::DatabaseBackend::Postgres }).await {
        Ok(result) => return Ok(result),
        Err(e) => panic!("{}", e.to_string())
    }
}

pub async fn create_database_if_not_exists() -> Result<(), sea_orm::DbErr> {
    let db_options = get_db_options();
    
    let maintenance = db_options.clone().database("postgres");
    let db: DatabaseConnection = Database::connect(maintenance.to_url_lossy()).await.unwrap();
    let database_to_create = db_options.get_database().unwrap();
    
    let check_if_exists_query = format!("SELECT 1 FROM pg_database WHERE datname = '{}'", database_to_create);
    let result = db.query_one(sea_orm::Statement::from_string(
        sea_orm::DatabaseBackend::Postgres,
        check_if_exists_query,
    )).await?;

    if result.is_none() {   
        match db.execute(sea_orm::Statement { sql: format!("create database {}", database_to_create), values: None, db_backend: sea_orm::DatabaseBackend::Postgres }).await {
            Ok(_result) => return Ok(()),
            Err(e) => panic!("{}", e.to_string())
        }
    } else {
        Ok(())
    }
}


#[tokio::main]
async fn main() {
    env_logger::init();
    info!("Checking if database exists ---");
    let _ = create_database_if_not_exists().await;

    info!("Running migrations ---");
    let db: DatabaseConnection = Database::connect(get_dsn()).await.unwrap();
    let _ = Migrator::up(&db, None).await;

    info!("Running seeder operations --- ");
    let _ = seeder::seed().await;

    let _ = pg_trgm_extension().await;
    
    info!("Running index operations --- ");
    let _ = search_index().await;
}
