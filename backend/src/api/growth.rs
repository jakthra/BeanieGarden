use entity::growth::Growth;
use leptos::prelude::*;

#[server]
pub async fn get_growths() -> Result<Vec<Growth>, ServerFnError> {
    use repositories::growth_repository::GrowthRepository;
    let repo = GrowthRepository::new();
    repo.get_all()
        .await
        .map_err(|e| ServerFnError::ServerError(e.to_string()))
}
