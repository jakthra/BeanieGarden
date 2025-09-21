use std::vec;

use entity::search::SearchResults;
use infra::get_dsn;
use migration::{Migrator, MigratorTrait};
use poem::middleware::{Cors, CorsEndpoint, Tracing, TracingEndpoint};
use poem::{EndpointExt, Route, listener::TcpListener};
use poem_openapi::Object;
use poem_openapi::{OpenApi, OpenApiService, param::Query, payload::Json};
use repositories::search_results_repository::SearchResultRepository;
use serde::{Deserialize, Serialize};
pub struct Api;

#[derive(Object, Serialize, Deserialize)]
pub struct CreateSearchRequest {
    pub q: String,
}

#[OpenApi]
impl Api {
    #[oai(path = "/search", method = "post")]
    async fn index(&self, req: Json<CreateSearchRequest>) -> Json<SearchResults> {
        let search_result_repository = SearchResultRepository::new();
        let results = search_result_repository.search(req.q.to_owned()).await;
        match results {
            Ok(results) => Json(results),
            Err(results) => Json(SearchResults { plants: vec![] }),
        }
    }
}

pub struct App {
    pub enable_swagger: bool,
    pub enable_openapi_spec: bool,
    pub host: String,
}

impl App {
    pub fn get_route(self) -> CorsEndpoint<TracingEndpoint<Route>> {
        let Self {
            enable_swagger,
            enable_openapi_spec,
            host,
        } = self;

        let api_service = OpenApiService::new(Api, "Hello World", "1.0").server(&host);

        let ui = api_service.swagger_ui();
        let spec = api_service.spec_endpoint();

        let mut route = Route::new().nest("/api", api_service);
        if enable_swagger {
            route = route.nest("/", ui);
        }
        if enable_openapi_spec {
            route = route.nest("/spec", spec)
        }
        route.with(Tracing).with(Cors::new())
    }
}
