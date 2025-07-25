use std::fs::{self, File};
use std::io::{BufReader, Write};

use entity::plant_species::Model;
use uuid::{NoContext, Timestamp, Uuid};

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, PartialEq)]
struct GenusSearch {
    q: String,
    family: String,
}

impl Default for GenusSearch {
    fn default() -> Self {
        GenusSearch {
            q: String::from(""),
            family: String::from(""),
        }
    }
}

#[derive(serde::Deserialize, serde::Serialize, Debug, PartialEq, Clone)]
struct CommonPlantSearch {
    genus_search: GenusSearch,
    common_danish_name: String,
    common_english_name: String,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
struct GenusResult {
    key: i64,
    scientificName: String,
    canonicalName: String,
    genus: String,
    #[serde(default)]
    family: String,
    rank: String,
    status: String,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
struct GenusGbifResult {
    common_plant_search: CommonPlantSearch,
    result: GenusResult,
}

const DIRNAME: &'static str = "gbif-results";
const BASE_GBIF_URL: &'static str = "https://api.gbif.org/v1/species/suggest?datasetKey=d7dddbf4-2cf0-4f39-9b2a-bb099caae36c&rank=GENUS&status=ACCEPTED";

#[derive(Debug)]
pub enum Error {
    Reqwest(reqwest::Error),
    NoResults,
    InpreciseResults,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Reqwest(e) => write!(f, "HTTP request error: {}", e),
            Error::NoResults => write!(f, "No results found for the given search"),
            Error::InpreciseResults => write!(f, "Inprecise results found for the given search"),
        }
    }
}

impl std::error::Error for Error {}

async fn query_gbif(search: &GenusSearch) -> Result<Vec<GenusResult>, Error> {
    let client = reqwest::Client::new();
    println!("Requesting data for {}", search.q);
    let result = client
        .get(BASE_GBIF_URL)
        .query(&[("q", search.q.clone())])
        .send()
        .await
        .map_err(Error::Reqwest)?
        .json::<Vec<GenusResult>>()
        .await
        .map_err(Error::Reqwest)?;

    if result.is_empty() {
        Err(Error::NoResults)
    } else {
        Ok(result)
    }
}

fn parse_gbif(search: &GenusSearch, results: &Vec<GenusResult>) -> Result<GenusResult, Error> {
    if results.len() == 1 {
        return Ok(results.first().unwrap().clone());
    } else {
        let curated_results: Vec<GenusResult> = results
            .clone()
            .into_iter()
            .filter(|r| r.family == search.family)
            .collect();

        if curated_results.len() == 0 {
            Err(Error::NoResults)
        } else if curated_results.len() == 1 {
            return Ok(curated_results.first().unwrap().clone());
        } else {
            println!(
                "Unprecise search for {} {} - improve it by adding additional filters.",
                search.q, search.family
            );
            let json_string = serde_json::to_string_pretty(&curated_results).unwrap();
            let file_name = format!(
                "{}/{}_{}.json",
                DIRNAME,
                search.q.to_owned(),
                search.family.to_owned()
            );
            let mut file = File::create(file_name.to_string()).unwrap();
            file.write_all(json_string.as_bytes()).unwrap();
            println!(
                "Successfully wrote raw {} results to {}",
                curated_results.len(),
                file_name
            );
            Err(Error::InpreciseResults)
        }
    }
}

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
        },
        CommonPlantSearch {
            genus_search: GenusSearch {
                q: "Cupressus".to_string(),
                family: "Cupressaceae".to_string(),
                ..Default::default()
            },
            common_danish_name: "Almindelig cypres".to_string(),
            common_english_name: "Cupressus sempervirens".to_string(),
        },
        CommonPlantSearch {
            genus_search: GenusSearch {
                q: "Prunus".to_string(),
                family: "Rosaceae".to_string(),
                ..Default::default()
            },
            common_danish_name: "Kirsebær".to_string(),
            common_english_name: "Cherry Blossom".to_string(),
        },
        CommonPlantSearch {
            genus_search: GenusSearch {
                q: "Rosa L.".to_string(),
                family: "Rosaceae".to_string(),
                ..Default::default()
            },
            common_danish_name: "Rose".to_string(),
            common_english_name: "Rose".to_string(),
        },
        CommonPlantSearch {
            genus_search: GenusSearch {
                q: "Salix".to_string(),
                family: "Salicaceae".to_string(),
                ..Default::default()
            },
            common_danish_name: "Japansk Pil".to_string(),
            common_english_name: "Salix".to_string(),
        },
        CommonPlantSearch {
            genus_search: GenusSearch {
                q: "Taxus".to_string(),
                family: "Taxaceae".to_string(),
                ..Default::default()
            },
            common_danish_name: "Taks".to_string(),
            common_english_name: "Yew".to_string(),
        },
        CommonPlantSearch {
            genus_search: GenusSearch {
                q: "Fagus".to_string(),
                family: "Fagaceae".to_string(),
                ..Default::default()
            },
            common_danish_name: "Bøg".to_string(),
            common_english_name: "Beech".to_string(),
        },
    ];
    let file_name = format!("{}/gbif-results.json", DIRNAME);
    let mut gbif_results: Vec<GenusGbifResult> = match File::open(file_name.to_owned()) {
        Ok(file) => {
            println!("Reading existing results from: {}", file_name);
            let reader = BufReader::new(file);
            serde_json::from_reader(reader).unwrap_or_else(|_| Vec::with_capacity(searches.len()))
        }
        Err(_) => Vec::with_capacity(searches.len()),
    };

    fs::create_dir_all(format!("./{}", DIRNAME)).unwrap();

    for search in searches.into_iter() {
        if gbif_results
            .iter()
            .any(|result| result.common_plant_search == search)
        {
            println!("Found {} in results. Skipping", search.common_english_name);
            continue;
        }
        let results = query_gbif(&search.genus_search).await.unwrap();
        let parsed_results = parse_gbif(&search.genus_search, &results);
        match parsed_results {
            Ok(results) => gbif_results.push(GenusGbifResult {
                common_plant_search: search,
                result: results,
            }),
            Err(err) => println!("Error parsing {:?}: {}", search, err),
        }
    }

    let json_string = serde_json::to_string_pretty(&gbif_results).unwrap();
    let mut file = File::create(file_name.to_string()).unwrap();
    file.write_all(json_string.as_bytes()).unwrap();
    println!("Results in: {}", file_name);

    // Convert to database objects. Read from json file (uuidv7 is used)
    // todo!();
    // let entities: Vec<Model> = gbif_results
    //     .iter()
    //     .map(|gbif_result| Model {
    //         uuid: Uuid::new_v7(Timestamp::now(NoContext)),
    //         canonical_name: gbif_result.result.canonicalName.to_owned(),
    //         family: gbif_result.result.family.to_owned(),
    //         gbif_key: gbif_result.result.key,
    //         genus: gbif_result.result.genus.to_owned(),
    //         scientic_name: gbif_result.result.scientificName.to_owned(),
    //     })
    //     .collect();

    Ok(())
}
