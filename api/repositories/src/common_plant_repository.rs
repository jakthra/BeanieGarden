use crate::repository::DatabaseRepository;
use entity::common_plant::CommonPlant;
use entity::gbif_genus::GbifGenus;
use sea_orm::ActiveValue::Set;
use sea_orm::EntityTrait;
use sea_orm::InsertResult;
use sea_orm::ModelTrait;
use sea_orm::sea_query::OnConflict;

pub struct CommonPlantRepository {
    repository: DatabaseRepository,
}

impl CommonPlantRepository {
    pub fn new() -> Self {
        Self {
            repository: DatabaseRepository::new(),
        }
    }

    pub async fn get(&self, key: i64) -> Result<CommonPlant, Box<dyn std::error::Error>> {
        let db = self.repository.connect().await?;
        let found: Option<common_plant::Model> =
            common_plant::Entity::find_by_id(key).one(&db).await?;

        let found = found.ok_or("CommonPlant not found")?;

        let gbif_genus: Option<gbif_genus::Model> =
            found.find_related(gbif_genus::Entity).one(&db).await?;

        let gbif_genus: gbif_genus::Model =
            gbif_genus.ok_or("Related GBIF Genus not found. Should never happen.")?;
        Ok(CommonPlant {
            common_danish_name: found.common_danish_name.to_owned(),
            common_english_name: found.common_english_name.to_owned(),
            da_wiki_url: found.da_wiki_url.to_owned(),
            description: found.description.to_owned(),
            image_url: found.image_url.to_owned(),

            gbif_genus: GbifGenus {
                canonical_name: gbif_genus.canonical_name,
                family: gbif_genus.family,
                genus: gbif_genus.genus,
                key: gbif_genus.key,
                rank: gbif_genus.rank,
                scientific_name: gbif_genus.scientific_name,
            },
        })
    }

    pub async fn add_many(
        &self,
        entities: Vec<CommonPlant>,
    ) -> Result<InsertResult<models::common_plant::ActiveModel>, Box<dyn std::error::Error>> {
        let db = self.repository.connect().await?;
        let results =
            common_plant::Entity::insert_many(entities.into_iter().map(|entity| ActiveModel {
                common_danish_name: Set(entity.common_danish_name),
                common_english_name: Set(entity.common_english_name),
                gbif_genus_key: Set(entity.gbif_genus.key),
                da_wiki_url: Set(entity.da_wiki_url),
                description: Set(entity.description),
                image_url: Set(entity.image_url),
                ..Default::default()
            }))
            .on_conflict(
                OnConflict::column(common_plant::Column::GbifGenusKey)
                    .update_columns([
                        common_plant::Column::CommonDanishName,
                        common_plant::Column::CommonEnglishName,
                        common_plant::Column::DaWikiUrl,
                        common_plant::Column::Description,
                        common_plant::Column::ImageUrl,
                    ])
                    .to_owned(),
            )
            .exec(&db)
            .await?;
        Ok(results)
    }
}
