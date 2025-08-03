use leptos::prelude::*;
use shared::Growth;

#[server]
pub async fn get_growths() -> Result<Vec<Growth>, ServerFnError> {
    use query_service::query_growths;
    Ok(query_growths().await.unwrap())
}
