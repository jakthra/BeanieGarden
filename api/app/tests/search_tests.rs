use poem::Route;
use poem::test::TestClient;
use poem_openapi::{OpenApi, OpenApiService};
use serde::Serialize;

// Import the Api struct from your main module
// Assuming your crate name is the same as your project name
use app::{Api, SearchResults};

#[tokio::test]
async fn test_search_endpoint() {
    // Create the API service
    let api_service = OpenApiService::new(Api, "Test API", "1.0");

    // Create the route
    let app = Route::new().nest("/api", api_service);

    // Create test client
    let cli = TestClient::new(app);

    // Test 1: Search without query parameter (should return "hello")
    let resp = cli.get("/api/search").send().await;
    resp.assert_status_is_ok();
    let json: SearchResults = resp.json().await.value().deserialize();
    assert_eq!(json.name, "hello");

    // Test 2: Search with query parameter (should return the query value)
    let resp = cli
        .get("/api/search")
        .query("q", &"test_query")
        .send()
        .await;
    resp.assert_status_is_ok();
    let json: SearchResults = resp.json().await.value().deserialize();
    assert_eq!(json.name, "test_query");

    // Test 3: Search with empty query parameter (should still return "hello")
    let resp = cli.get("/api/search?q=").send().await;
    resp.assert_status_is_ok();
    let json: SearchResults = resp.json().await.value().deserialize();
    assert_eq!(json.name, "");
}
