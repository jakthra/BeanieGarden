use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(GardeningTask::Table)
                    .if_not_exists()
                    .col(pk_uuid(GardeningTask::Uuid))
                    .col(string(GardeningTask::Title))
                    .col(string(GardeningTask::Priority))
                    .col(float(GardeningTask::TimeRequired))
                    .col(string(GardeningTask::Description))
                    .col(string(GardeningTask::Tips))
                    .col(uuid(GardeningTask::AccountUuid))
                    .to_owned(),
            )
            .await
            .unwrap();

        manager
            .create_table(
                Table::create()
                    .table(GbifGenus::Table)
                    .if_not_exists()
                    .col(pk_auto(GbifGenus::Id))
                    .col(string_uniq(GbifGenus::Key))
                    .col(string(GbifGenus::CanonicalName))
                    .col(string(GbifGenus::ScientificName))
                    .col(string(GbifGenus::Family))
                    .col(string(GbifGenus::Genus))
                    .col(string(GbifGenus::Rank))
                    .to_owned(),
            )
            .await
            .unwrap();

        manager
            .create_table(
                Table::create()
                    .table(CommonPlant::Table)
                    .if_not_exists()
                    .col(pk_auto(CommonPlant::Id))
                    .col(string_uniq(CommonPlant::CommonDanishName))
                    .col(string_uniq(CommonPlant::CommonEnglishName))
                    .col(integer(CommonPlant::GbifGenusId))
                    .comment("Common derivatives of 'plants'. Uses genus descriptors from GBIF.")
                    .to_owned(),
            )
            .await
            .unwrap();

        manager
            .create_table(
                Table::create()
                    .table(Account::Table)
                    .if_not_exists()
                    .col(pk_uuid(Account::Uuid))
                    .col(string(Account::Email))
                    .col(boolean(Account::Active))
                    .comment("Default Account table")
                    .to_owned(),
            )
            .await
            .unwrap();

        manager
            .create_table(
                Table::create()
                    .table(Growth::Table)
                    .if_not_exists()
                    .col(pk_uuid(Growth::Uuid))
                    .col(string(Growth::GrowthType))
                    .col(float(Growth::AgeEstimate))
                    .col(float(Growth::Height))
                    .col(float(Growth::Width))
                    .col(integer(Growth::CommonPlantId))
                    .col(uuid(Growth::AccountUuid))
                    .to_owned(),
            )
            .await
            .unwrap();

        manager
            .create_table(
                Table::create()
                    .table(GardeningTaskGrowthAssoication::Table)
                    .if_not_exists()
                    .col(pk_uuid(GardeningTaskGrowthAssoication::Uuid))
                    .col(uuid(GardeningTaskGrowthAssoication::GardeningTaskUuid))
                    .col(uuid(GardeningTaskGrowthAssoication::GrowthUuid))
                    .to_owned(),
            )
            .await
            .unwrap();

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .from(CommonPlant::Table, CommonPlant::Id)
                    .to(GbifGenus::Table, GbifGenus::Id)
                    .to_owned(),
            )
            .await
            .unwrap();
        manager
            .create_foreign_key(
                ForeignKey::create()
                    .from(CommonPlant::Table, CommonPlant::GbifGenusId)
                    .to(GbifGenus::Table, GbifGenus::Id)
                    .to_owned(),
            )
            .await
            .unwrap();

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .from(
                        GardeningTaskGrowthAssoication::Table,
                        GardeningTaskGrowthAssoication::GrowthUuid,
                    )
                    .to(Growth::Table, Growth::Uuid)
                    .to_owned(),
            )
            .await
            .unwrap();

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .from(
                        GardeningTaskGrowthAssoication::Table,
                        GardeningTaskGrowthAssoication::GardeningTaskUuid,
                    )
                    .to(GardeningTask::Table, GardeningTask::Uuid)
                    .to_owned(),
            )
            .await
            .unwrap();

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .from(Growth::Table, Growth::AccountUuid)
                    .to(Account::Table, Account::Uuid)
                    .to_owned(),
            )
            .await
            .unwrap();

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .from(Growth::Table, Growth::CommonPlantId)
                    .to(CommonPlant::Table, CommonPlant::Id)
                    .to_owned(),
            )
            .await
            .unwrap();

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .from(GardeningTask::Table, GardeningTask::AccountUuid)
                    .to(Account::Table, Account::Uuid)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .table(GardeningTaskGrowthAssoication::Table)
                    .to_owned(),
            )
            .await
            .unwrap();
        manager
            .drop_table(Table::drop().table(Growth::Table).to_owned())
            .await
            .unwrap();
        manager
            .drop_table(Table::drop().table(GardeningTask::Table).to_owned())
            .await
            .unwrap();
        manager
            .drop_table(Table::drop().table(CommonPlant::Table).to_owned())
            .await
            .unwrap();
        manager
            .drop_table(Table::drop().table(GbifGenus::Table).to_owned())
            .await
            .unwrap();

        manager
            .drop_table(Table::drop().table(Account::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum GardeningTask {
    Table,
    Uuid,
    Title,
    Priority,
    TimeRequired,
    Description,
    Tips,
    AccountUuid,
}

#[derive(DeriveIden)]
enum GardeningTaskGrowthAssoication {
    Table,
    Uuid,
    GardeningTaskUuid,
    GrowthUuid,
}

#[derive(DeriveIden)]
pub enum GbifGenus {
    Table,
    Id,
    Key,
    ScientificName,
    CanonicalName,
    Genus,
    Rank,
    Family,
}

#[derive(DeriveIden)]
pub enum CommonPlant {
    Table,
    Id,
    CommonDanishName,
    CommonEnglishName,
    GbifGenusId,
}

#[derive(DeriveIden)]
pub enum Growth {
    Table,
    Uuid,
    GrowthType,
    AgeEstimate,
    Height,
    Width,
    CommonPlantId,
    AccountUuid,
}

#[derive(DeriveIden)]
enum Account {
    Table,
    Uuid,
    Email,
    Active,
}
