use infra::postgres::get_dsn;
use sea_orm::{Database, DatabaseConnection, DbErr};

pub struct DatabaseRepository {
    dsn: String,
}

impl DatabaseRepository {
    pub fn new() -> Self {
        Self {
            dsn: get_dsn().to_string(),
        }
    }

    pub async fn connect(&self) -> Result<DatabaseConnection, DbErr> {
        Database::connect(self.dsn.to_owned()).await
    }
}

impl Default for DatabaseRepository {
    fn default() -> Self {
        Self::new()
    }
}
