#![allow(non_snake_case)]
use models::account;
use models::common_plant;
use models::gbif_genus;
use models::gbif_genus::Column;
use sea_orm::ActiveValue::Set;
use sea_orm::ColumnTrait;
use sea_orm::EntityTrait;
use sea_orm::QueryFilter;
use sea_orm::sea_query::OnConflict;
use sea_orm::{Database, DatabaseConnection};
use uuid::NoContext;
use uuid::Timestamp;
use uuid::Uuid;

use crate::gbif_service::*;

pub mod gbif_service;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let searches = vec![
        CommonPlantSearch {
            genus_search: GenusSearch {
                q: "Rhododendron".to_string(),
                family: "Ericaceae".to_string(),
                ..Default::default()
            },
            common_danish_name: "Rhodondendron".to_string(),
            common_english_name: "Rhodondendron".to_string(),
            da_wiki_url: "https://da.wikipedia.org/wiki/Rododendron".to_string(),
            image_url: "https://upload.wikimedia.org/wikipedia/commons/7/75/Rhododendron-by-eiffel-public-domain-20040617.jpg".to_string(),
            description: "".to_string(),
        },
        CommonPlantSearch {
            genus_search: GenusSearch {
                q: "Cupressus".to_string(),
                family: "Cupressaceae".to_string(),
                ..Default::default()
            },
            common_danish_name: "Almindelig cypres".to_string(),
            common_english_name: "Cupressus sempervirens".to_string(),
            da_wiki_url: "https://da.wikipedia.org/wiki/Almindelig_cypres".to_string(),
            image_url: "https://upload.wikimedia.org/wikipedia/commons/b/b1/Med_Cypress.jpg".to_string(),
            description: "".to_string()
        },
        CommonPlantSearch {
            genus_search: GenusSearch {
                q: "Prunus".to_string(),
                family: "Rosaceae".to_string(),
                ..Default::default()
            },
            common_danish_name: "Kirsebær".to_string(),
            common_english_name: "Cherry Blossom".to_string(),
            da_wiki_url: "https://da.wikipedia.org/wiki/Japansk_kirseb%C3%A6r".to_string(),
            image_url: "https://da.wikipedia.org/wiki/Japansk_kirseb%C3%A6r#/media/Fil:Prunus-serrulata-cultivar-9.JPG".to_string(),
            description: "".to_string()
        },
        CommonPlantSearch {
            genus_search: GenusSearch {
                q: "Rosa L.".to_string(),
                family: "Rosaceae".to_string(),
                ..Default::default()
            },
            common_danish_name: "Rose".to_string(),
            common_english_name: "Rose".to_string(),
            da_wiki_url: "https://da.wikipedia.org/wiki/Rose".to_string(),
            image_url: "https://da.wikipedia.org/wiki/Rose#/media/Fil:Rosa_rubiginosa_1.jpg".to_string(),
            description: "".to_string()
        },
        CommonPlantSearch {
            genus_search: GenusSearch {
                q: "Salix".to_string(),
                family: "Salicaceae".to_string(),
                ..Default::default()
            },
            common_danish_name: "Japansk Pil".to_string(),
            common_english_name: "Salix".to_string(),
            da_wiki_url: "https://da.wikipedia.org/wiki/Pile-sl%C3%A6gten".to_string(),
            image_url: "https://da.wikipedia.org/wiki/Pile-sl%C3%A6gten#/media/Fil:Salix-catkins.JPG".to_string(),
            description: "".to_string()
        },
        CommonPlantSearch {
            genus_search: GenusSearch {
                q: "Taxus".to_string(),
                family: "Taxaceae".to_string(),
                ..Default::default()
            },
            common_danish_name: "Taks".to_string(),
            common_english_name: "Yew".to_string(),
            da_wiki_url: "https://da.wikipedia.org/wiki/Almindelig_taks".to_string(),
            image_url: "https://upload.wikimedia.org/wikipedia/commons/3/31/Taxus_baccata_MHNT.jpg".to_string(),
            description: "".to_string()
        },
        CommonPlantSearch {
            genus_search: GenusSearch {
                q: "Fagus".to_string(),
                family: "Fagaceae".to_string(),
                ..Default::default()
            },
            common_danish_name: "Bøg".to_string(),
            common_english_name: "Beech".to_string(),
            da_wiki_url: "https://da.wikipedia.org/wiki/Almindelig_b%C3%B8g".to_string(),
            image_url: "https://upload.wikimedia.org/wikipedia/commons/a/ad/Fagus-sylvatica-DK.JPG".to_string(),
            description: "".to_string()
        },
    ];

    let gbif_service = GbifService::new();
    let gbif_results = gbif_service.search(searches).await.unwrap();

    let common_plants: Vec<common_plant::ActiveModel> = gbif_results
        .iter()
        .map(|r| common_plant::ActiveModel {
            gbif_genus_key: Set(r.result.key),
            common_danish_name: Set(r.common_plant_search.common_danish_name.to_owned()),
            common_english_name: Set(r.common_plant_search.common_english_name.to_owned()),
            image_url: Set(r.common_plant_search.image_url.to_owned()),
            da_wiki_url: Set(r.common_plant_search.da_wiki_url.to_owned()),
            decription: Set(r.common_plant_search.description.to_owned()),
            ..Default::default()
        })
        .collect();

    println!(
        "Number of common_plant records to insert: {}",
        common_plants.len()
    );

    let gbif_genus_results: Vec<gbif_genus::ActiveModel> = gbif_results
        .iter()
        .map(|r| gbif_genus::ActiveModel {
            canonical_name: Set(r.result.canonicalName.to_owned()),
            family: Set(r.result.family.to_owned()),
            genus: Set(r.result.genus.to_owned()),
            key: Set(r.result.key.to_owned()),
            rank: Set(r.result.rank.to_owned()),
            scientific_name: Set(r.result.scientificName.to_owned()),
            ..Default::default()
        })
        .collect();

    let db: DatabaseConnection = Database::connect(get_dsn()).await.unwrap();

    gbif_genus::Entity::insert_many(gbif_genus_results)
        .on_conflict(
            OnConflict::column(Column::Key)
                .update_columns([
                    Column::CanonicalName,
                    Column::Family,
                    Column::Genus,
                    Column::Rank,
                    Column::ScientificName,
                ])
                .to_owned(),
        )
        .exec(&db)
        .await
        .unwrap();

    common_plant::Entity::insert_many(common_plants)
        .on_conflict(
            OnConflict::column(common_plant::Column::GbifGenusKey)
                .update_columns([
                    common_plant::Column::CommonDanishName,
                    common_plant::Column::CommonEnglishName,
                ])
                .to_owned(),
        )
        .exec(&db)
        .await
        .unwrap();
    println!("Successfully inserted all records.");

    // Create default account, and a few default growths
    let account = account::Entity::find()
        .filter(account::Column::Email.eq("admin@beaniegeanie.io"))
        .one(&db)
        .await
        .unwrap();

    let account_id = if let Some(account) = account {
        account.uuid
    } else {
        let new_account = account::ActiveModel {
            email: Set("admin@beaniegeanie.io".to_string()),
            uuid: Set(Uuid::new_v7(Timestamp::now(NoContext))),
            ..Default::default()
        };
        let result = account::Entity::insert(new_account)
            .exec(&db)
            .await
            .unwrap();
        result.last_insert_id
    };

    Ok(())
}
