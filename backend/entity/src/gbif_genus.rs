use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GbifGenus {
    pub key: i64,
    pub canonical_name: String,
    pub scientific_name: String,
    pub family: String,
    pub genus: String,
    pub rank: String,
}
