use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::common_plant::CommonPlant;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Growth {
    pub uuid: Uuid,
    pub growth_type: String,
    pub age_estimate: f32,
    pub height: f32,
    pub width: f32,
    pub common_plant: CommonPlant,
}
