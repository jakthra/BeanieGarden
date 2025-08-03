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
                    .col(big_integer_uniq(GbifGenus::Key))
                    .primary_key(Index::create().col(GbifGenus::Key))
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
                    .col(string_uniq(CommonPlant::CommonDanishName))
                    .col(string_uniq(CommonPlant::CommonEnglishName))
                    .col(big_integer_uniq(CommonPlant::GbifGenusKey))
                    .primary_key(Index::create().col(CommonPlant::GbifGenusKey))
                    .comment(
                        "Common derivatives of 'plants'. Uses genus descriptors from GBIF."
                            .to_string(),
                    )
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
                    .col(
                        ColumnDef::new(Account::CreatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Account::Active)
                            .boolean()
                            .not_null()
                            .default(true),
                    )
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
                    .col(
                        ColumnDef::new(Growth::CreatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Growth::Active)
                            .boolean()
                            .not_null()
                            .default(true),
                    )
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
                    .from(CommonPlant::Table, CommonPlant::GbifGenusKey)
                    .to(GbifGenus::Table, GbifGenus::Key)
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
                    .to(CommonPlant::Table, CommonPlant::GbifGenusKey)
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
    CommonDanishName,
    CommonEnglishName,
    GbifGenusKey,
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
    CreatedAt,
    Active,
}

#[derive(DeriveIden)]
enum Account {
    Table,
    Uuid,
    Email,
    CreatedAt,
    Active,
}
