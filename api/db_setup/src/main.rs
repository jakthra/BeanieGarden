
use seeder::seed;

#[tokio::main]
async fn main() {
    let _ = seed().await;
}
