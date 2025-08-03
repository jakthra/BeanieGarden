use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GbifGenus {
    pub key: i64,
    pub canonical_name: String,
    pub scientific_name: String,
    pub family: String,
    pub genus: String,
    pub rank: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CommonPlant {
    pub common_danish_name: String,
    pub common_english_name: String,
    pub gbif_genus: GbifGenus,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Growth {
    pub uuid: Uuid,
    pub growth_type: String,
    pub age_estimate: f32,
    pub height: f32,
    pub width: f32,
    pub common_plant: CommonPlant,
}
