use crate::repository::DatabaseRepository;
use entity::common_plant::CommonPlant;
use entity::gbif_genus::GbifGenus;
use entity::growth::Growth;
use models::common_plant;
use models::gbif_genus;
use models::growth::{self, ActiveModel};
use sea_orm::ActiveValue::Set;
use sea_orm::ColumnTrait;
use sea_orm::EntityTrait;
use sea_orm::ModelTrait;
use sea_orm::QueryFilter;
use uuid::Uuid;

pub struct SearchRepository {
    repository: DatabaseRepository,
}

impl SearchRepository {
    pub fn new() -> Self {
        Self {
            repository: DatabaseRepository::new(),
        }
    }
}
