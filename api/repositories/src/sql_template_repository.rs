use std::{fs, path::Path, task};
use strum_macros::AsRefStr;
use crate::repository::FolderRepository;
use minijinja::{Environment, context};
use serde::Serialize;

pub struct SQLTemplateRepository {
    repository: FolderRepository,
}

#[derive(AsRefStr)]
pub enum SQLTemplates {
    #[strum(serialize = "search_query.sql")]
    SearchQuery,
}



impl SQLTemplateRepository {
    pub fn new() -> Self {
        Self {
            repository: FolderRepository::new(),
        }
    }
    pub fn render<S: Serialize>(&self, sql_file: SQLTemplates, context: S) -> String {
        let mut env = Environment::new();
        let sql_file = sql_file.as_ref().to_string();
        let search_template = self.repository.read(&sql_file);
        env.add_template(&sql_file, &search_template).unwrap();
        let template = env.get_template(&sql_file).unwrap();
        template.render(context).unwrap()

    }
}