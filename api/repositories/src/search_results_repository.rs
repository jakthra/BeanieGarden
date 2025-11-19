use std::collections::HashMap;
use std::{fs, path::Path};

use entity::search::{PlantRecord, SearchResults};
use sea_orm::{DbErr};
use minijinja::{Environment, context};
use crate::repository::DatabaseRepository;
use crate::sql_template_repository::{SQLTemplateRepository, SQLTemplates};


pub struct SearchResultRepository {
    repository: DatabaseRepository,
}

pub fn get_search_template() -> Result<String, std::io::Error> {
    let sql_dir = "sql";
    let sql_index_file = "search_index.sql";
    let path = Path::new(&sql_dir).join(sql_index_file);
    println!("{:?}", path);
    match fs::read_to_string(&path) {
        Ok(sql) => return Ok(sql),
        Err(e) => return Err(e)
    }
}

impl SearchResultRepository {
    pub fn new() -> Self {
        Self {
            repository: DatabaseRepository::new(),
        }
    }

    pub fn query(&self, q: String) -> String {
        let mut context: HashMap<String, String> = HashMap::new();
        context.insert("search_term".to_string(), q);
        let sql_repository = SQLTemplateRepository::new();
        sql_repository.render(SQLTemplates::SearchQuery, context)
    } 

    pub async fn search(&self, q: String) -> Result<SearchResults, DbErr> {
        // let db = self.repository.connect().await?;

        Ok(SearchResults {
            plants: [PlantRecord {
                common_name: "test".to_string(),
                description: "test".to_string(),
                family: "test".to_string(),
                image_url: "test".to_string(),
                in_garden: true,
                wiki_url: "test".to_string(),
            }]
            .to_vec(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_sql() {
        assert!(SearchResultRepository::new().query("test".to_string()).to_lowercase().contains("select"))
    }
}