use entity::common_plant;
use entity::gbif_genus;
use entity::growth;
use infra::get_dsn;
use sea_orm::SqlxError;
use sea_orm::{Database, DatabaseConnection, EntityTrait, QueryOrder};
use shared::CommonPlant;
use shared::GbifGenus;
use shared::Growth;

pub async fn query_growths() -> Result<Vec<Growth>, SqlxError> {
    let db: DatabaseConnection = Database::connect(get_dsn()).await.unwrap();
    let growths_with_plants: Vec<(
        growth::Model,
        Option<common_plant::Model>,
        Option<gbif_genus::Model>,
    )> = growth::Entity::find()
        .find_also_related(common_plant::Entity)
        .and_also_related(gbif_genus::Entity)
        .order_by_asc(growth::Column::Uuid)
        .all(&db)
        .await
        .unwrap();

    let results: Vec<Growth> = growths_with_plants
        .into_iter()
        .map(|(growth_model, common_plants, gbif_genus)| {
            // Use .expect() since you know they exist
            let plant = common_plants.expect("Plant relation should exist");
            let genus = gbif_genus.expect("Genus relation should exist");

            Growth {
                uuid: growth_model.uuid,
                growth_type: growth_model.growth_type,
                age_estimate: growth_model.age_estimate,
                height: growth_model.height,
                width: growth_model.width,
                common_plant: CommonPlant {
                    // Direct access!
                    common_danish_name: plant.common_danish_name,
                    common_english_name: plant.common_english_name,
                    gbif_genus: GbifGenus {
                        key: genus.key,
                        canonical_name: genus.canonical_name,
                        scientific_name: genus.scientific_name,
                        family: genus.family,
                        genus: genus.genus,
                        rank: genus.rank,
                    },
                },
            }
        })
        .collect();

    Ok(results)
}
