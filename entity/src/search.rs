#[derive(Clone)]
pub struct PlantRecord {
    pub common_name: String,
    pub family: String,
    pub wiki_url: String,
    pub image_url: String,
    pub description: String,
    pub in_garden: bool,
}

#[derive(Clone)]
pub struct SearchResults {
    pub plants: Vec<PlantRecord>,
}
