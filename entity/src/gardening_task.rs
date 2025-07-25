//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.0

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "gardening_task")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub uuid: Uuid,
    pub title: String,
    pub priority: String,
    #[sea_orm(column_type = "Float")]
    pub time_required: f32,
    pub description: String,
    pub tips: String,
    pub account_uuid: Uuid,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::account::Entity",
        from = "Column::AccountUuid",
        to = "super::account::Column::Uuid",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Account,
    #[sea_orm(has_many = "super::gardening_task_growth_assoication::Entity")]
    GardeningTaskGrowthAssoication,
}

impl Related<super::account::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Account.def()
    }
}

impl Related<super::gardening_task_growth_assoication::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::GardeningTaskGrowthAssoication.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
