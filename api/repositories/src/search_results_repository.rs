use entity::search::{PlantRecord, SearchResults};
use sea_orm::DbErr;

use crate::repository::DatabaseRepository;

pub struct SearchResultRepository {
    repository: DatabaseRepository,
}

impl SearchResultRepository {
    pub fn new() -> Self {
        Self {
            repository: DatabaseRepository::new(),
        }
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
