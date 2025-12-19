use crate::repository::DatabaseRepository;
use entity::common_plant::CommonPlant;
use entity::gbif_genus::GbifGenus;
use entity::growth::Growth;
use sea_orm::ActiveValue::Set;
use sea_orm::ColumnTrait;
use sea_orm::EntityTrait;
use sea_orm::ModelTrait;
use sea_orm::QueryFilter;
use uuid::Uuid;

pub struct GrowthRepository {
    repository: DatabaseRepository,
}

impl GrowthRepository {
    pub fn new() -> Self {
        Self {
            repository: DatabaseRepository::new(),
        }
    }

    pub async fn get(&self, uuid: Uuid) -> Result<Growth, Box<dyn std::error::Error>> {
        let db = self.repository.connect().await?;
        let found: Option<growth::Model> = growth::Entity::find_by_id(uuid).one(&db).await?;

        let found = found.ok_or("Growth not found")?;

        let common_plant: Option<common_plant::Model> =
            found.find_related(common_plant::Entity).one(&db).await?;

        let common_plant: common_plant::Model =
            common_plant.ok_or("Related CommonPlant not found. Should never happen.")?;

        let gbif_genus: Option<gbif_genus::Model> = common_plant
            .find_related(gbif_genus::Entity)
            .one(&db)
            .await?;

        let gbif_genus: gbif_genus::Model =
            gbif_genus.ok_or("Related GBIF Genus not found. Should never happen.")?;

        Ok(Growth {
            uuid: found.uuid,
            growth_type: found.growth_type,
            age_estimate: found.age_estimate,
            height: found.height,
            width: found.width,
            common_plant: CommonPlant {
                common_danish_name: common_plant.common_danish_name,
                common_english_name: common_plant.common_english_name,
                da_wiki_url: common_plant.da_wiki_url,
                description: common_plant.description,
                image_url: common_plant.image_url,
                gbif_genus: GbifGenus {
                    canonical_name: gbif_genus.canonical_name,
                    family: gbif_genus.family,
                    genus: gbif_genus.genus,
                    key: gbif_genus.key,
                    rank: gbif_genus.rank,
                    scientific_name: gbif_genus.scientific_name,
                },
            },
        })
    }

    pub async fn add(
        &self,
        growth: Growth,
        created_by: Uuid,
    ) -> Result<growth::Model, Box<dyn std::error::Error>> {
        let db = self.repository.connect().await?;

        let active_model = ActiveModel {
            uuid: Set(growth.uuid),
            growth_type: Set(growth.growth_type),
            age_estimate: Set(growth.age_estimate),
            height: Set(growth.height),
            width: Set(growth.width),
            common_plant_id: Set(growth.common_plant.gbif_genus.key),
            created_by: Set(created_by),
            created_at: Set(chrono::Utc::now().naive_utc()),
            active: Set(true),
        };

        let result = growth::Entity::insert(active_model).exec(&db).await?;
        let inserted = growth::Entity::find_by_id(result.last_insert_id)
            .one(&db)
            .await?;

        inserted.ok_or("Failed to retrieve inserted growth".into())
    }

    pub async fn get_by_common_plant(
        &self,
        common_plant_id: i64,
    ) -> Result<Vec<Growth>, Box<dyn std::error::Error>> {
        let db = self.repository.connect().await?;
        let growths: Vec<growth::Model> = growth::Entity::find()
            .filter(growth::Column::CommonPlantId.eq(common_plant_id))
            .filter(growth::Column::Active.eq(true))
            .all(&db)
            .await?;

        let mut result = Vec::new();
        for growth_model in growths {
            let common_plant: Option<common_plant::Model> = growth_model
                .find_related(common_plant::Entity)
                .one(&db)
                .await?;

            let common_plant: common_plant::Model =
                common_plant.ok_or("Related CommonPlant not found. Should never happen.")?;

            let gbif_genus: Option<gbif_genus::Model> = common_plant
                .find_related(gbif_genus::Entity)
                .one(&db)
                .await?;

            let gbif_genus: gbif_genus::Model =
                gbif_genus.ok_or("Related GBIF Genus not found. Should never happen.")?;

            result.push(Growth {
                uuid: growth_model.uuid,
                growth_type: growth_model.growth_type,
                age_estimate: growth_model.age_estimate,
                height: growth_model.height,
                width: growth_model.width,
                common_plant: CommonPlant {
                    common_danish_name: common_plant.common_danish_name,
                    common_english_name: common_plant.common_english_name,
                    da_wiki_url: common_plant.da_wiki_url,
                    description: common_plant.description,
                    image_url: common_plant.image_url,
                    gbif_genus: GbifGenus {
                        canonical_name: gbif_genus.canonical_name,
                        family: gbif_genus.family,
                        genus: gbif_genus.genus,
                        key: gbif_genus.key,
                        rank: gbif_genus.rank,
                        scientific_name: gbif_genus.scientific_name,
                    },
                },
            });
        }

        Ok(result)
    }

    pub async fn get_all(&self) -> Result<Vec<Growth>, Box<dyn std::error::Error>> {
        let db = self.repository.connect().await?;
        let growths: Vec<growth::Model> = growth::Entity::find()
            .filter(growth::Column::Active.eq(true))
            .all(&db)
            .await?;

        let mut result = Vec::new();
        for growth_model in growths {
            let common_plant: Option<common_plant::Model> = growth_model
                .find_related(common_plant::Entity)
                .one(&db)
                .await?;

            let common_plant: common_plant::Model =
                common_plant.ok_or("Related CommonPlant not found. Should never happen.")?;

            let gbif_genus: Option<gbif_genus::Model> = common_plant
                .find_related(gbif_genus::Entity)
                .one(&db)
                .await?;

            let gbif_genus: gbif_genus::Model =
                gbif_genus.ok_or("Related GBIF Genus not found. Should never happen.")?;

            result.push(Growth {
                uuid: growth_model.uuid,
                growth_type: growth_model.growth_type,
                age_estimate: growth_model.age_estimate,
                height: growth_model.height,
                width: growth_model.width,
                common_plant: CommonPlant {
                    common_danish_name: common_plant.common_danish_name,
                    common_english_name: common_plant.common_english_name,
                    da_wiki_url: common_plant.da_wiki_url,
                    description: common_plant.description,
                    image_url: common_plant.image_url,
                    gbif_genus: GbifGenus {
                        canonical_name: gbif_genus.canonical_name,
                        family: gbif_genus.family,
                        genus: gbif_genus.genus,
                        key: gbif_genus.key,
                        rank: gbif_genus.rank,
                        scientific_name: gbif_genus.scientific_name,
                    },
                },
            });
        }

        Ok(result)
    }
}
