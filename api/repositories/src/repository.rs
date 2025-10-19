use std::{fs, path::{Path, PathBuf}};

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


pub struct FolderRepository  {
    pub path: PathBuf,
}


impl FolderRepository {
    pub fn new() -> Self {
        Self {
            path: PathBuf::from("/home/jakob/Documents/beaniegarden/api/sql")
        }
    }

    pub fn get(&self, filename: String) -> PathBuf {
        self.path.join(filename)
    }

    pub fn read(&self, filename: &String) -> String {
        fs::read_to_string(self.get(filename.to_owned())).expect("File not found")
    }
}