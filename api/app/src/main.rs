use app::{Api, App};
use infra::postgres::get_dsn;
use migration::{Migrator, MigratorTrait};
use poem::listener::TcpListener;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    tracing_subscriber::fmt()
        .with_env_filter("poem=trace")
        .init();
    let connection = sea_orm::Database::connect(get_dsn()).await.unwrap();
    let _ = Migrator::up(&connection, None).await;

    let app = App {
        enable_openapi_spec: true,
        enable_swagger: true,
        host: "http://localhost:3000/api".to_string(),
    };

    poem::Server::new(TcpListener::bind("0.0.0.0:3000"))
        .run(app.get_route())
        .await
}
