use serde::{Deserialize, Serialize};

use crate::gbif_genus::GbifGenus;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CommonPlant {
    pub common_danish_name: String,
    pub common_english_name: String,
    pub gbif_genus: GbifGenus,
    pub da_wiki_url: String,
    pub image_url: String,
    pub description: String,
}
