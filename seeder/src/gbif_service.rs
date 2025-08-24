use std::fs;
use std::fs::File;
use std::io::{BufReader, Write};
const BASE_GBIF_URL: &'static str = "https://api.gbif.org/v1/species/suggest?datasetKey=d7dddbf4-2cf0-4f39-9b2a-bb099caae36c&rank=GENUS&status=ACCEPTED";
const DIRNAME: &'static str = "gbif-results";

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, PartialEq)]
pub struct GenusSearch {
    pub q: String,
    pub family: String,
}

impl Default for GenusSearch {
    fn default() -> Self {
        GenusSearch {
            q: String::from(""),
            family: String::from(""),
        }
    }
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct GenusResult {
    pub key: i64,
    pub scientificName: String,
    pub canonicalName: String,
    pub genus: String,
    #[serde(default)]
    pub family: String,
    pub rank: String,
    pub status: String,
}

pub struct GbifService {
    client: reqwest::Client,
    pub base_url: String,
    pub dirname: String,
    pub results_output: String,
}

#[derive(Debug)]
pub enum Error {
    Reqwest(reqwest::Error),
    NoResults,
    InpreciseResults,
    Parsing(String),
    SerdeJson(serde_json::Error),
}

#[derive(Debug)]
pub enum GbifError {
    SearchError(Vec<Error>),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Reqwest(e) => write!(f, "HTTP request error: {}", e),
            Error::NoResults => write!(f, "No results found for the given search"),
            Error::InpreciseResults => write!(f, "Inprecise results found for the given search"),
            Error::Parsing(s) => write!(f, "Parsing error found {}", s),
            Error::SerdeJson(e) => write!(f, "SerdeJSON error: {}", e),
        }
    }
}

impl std::error::Error for Error {}

#[derive(serde::Deserialize, serde::Serialize, Debug, PartialEq, Clone)]
pub struct CommonPlantSearch {
    pub genus_search: GenusSearch,
    pub common_danish_name: String,
    pub common_english_name: String,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct GenusGbifResult {
    pub common_plant_search: CommonPlantSearch,
    pub result: GenusResult,
}

impl GbifService {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
            base_url: BASE_GBIF_URL.to_string(),
            dirname: DIRNAME.to_string(),
            results_output: format!("{}/gbif-results.json", DIRNAME.to_string()),
        }
    }

    pub async fn query(&self, search: &GenusSearch) -> Result<Vec<GenusResult>, Error> {
        println!("Requesting data for {}", search.q);
        let result = self
            .client
            .get(self.base_url.to_owned())
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

    pub fn load_results(&self, expected_results: usize) -> Result<Vec<GenusGbifResult>, Error> {
        let results_output = self.results_output.to_owned();
        let file = match File::open(&results_output) {
            Ok(file) => file,
            Err(_) => {
                return Ok(Vec::with_capacity(expected_results));
            }
        };
        println!("Reading existing results from: {}", results_output);
        let reader = BufReader::new(file);

        match serde_json::from_reader(reader) {
            Ok(results) => Ok(results),
            Err(e) => Err(Error::SerdeJson(e)),
        }
    }

    pub async fn search(
        &self,
        searches: Vec<CommonPlantSearch>,
    ) -> Result<Vec<GenusGbifResult>, GbifError> {
        let mut gbif_results: Vec<GenusGbifResult> = self.load_results(searches.len()).unwrap();
        fs::create_dir_all(format!("./{}", self.dirname)).unwrap();
        let mut errors: Vec<Error> = Vec::with_capacity(searches.len());
        for search in searches.into_iter() {
            if gbif_results
                .iter()
                .any(|result| result.common_plant_search == search)
            {
                println!("Found {} in results. Skipping", search.common_english_name);
                continue;
            }
            let results = self.query(&search.genus_search).await.unwrap();
            let parsed_results = self.parse_gbif(&search.genus_search, &results);
            match parsed_results {
                Ok(results) => gbif_results.push(GenusGbifResult {
                    common_plant_search: search,
                    result: results,
                }),
                Err(err) => errors.push(Error::Parsing(format!(
                    "Error parsing {:?}: {}",
                    search, err
                ))),
            }
        }

        let json_string = serde_json::to_string_pretty(&gbif_results).unwrap();
        let mut file = File::create(self.results_output.to_string()).unwrap();
        file.write_all(json_string.as_bytes()).unwrap();
        println!("Results in: {}", self.results_output);

        if errors.is_empty() {
            Ok(gbif_results)
        } else {
            Err(GbifError::SearchError(errors))
        }
    }

    pub fn parse_gbif(
        &self,
        search: &GenusSearch,
        results: &Vec<GenusResult>,
    ) -> Result<GenusResult, Error> {
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
                    self.dirname.to_string(),
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
}
