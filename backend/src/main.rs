mod api;
mod margin;

use axum::routing::{get, post};
use axum::Router;
use std::net::SocketAddr;
use sqlx::PgPool;
use dotenvy::dotenv;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Load .env
    dotenv().ok();

    // Read DATABASE_URL
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env");

    // Create Postgres connection pool
    let pool = PgPool::connect(&database_url).await?;
    println!("✅ Connected to Postgres");

    // Create router and routes
    let app = Router::new()
        .route("/health", get(health_check))
        .route("/positions/open", post(api::open_position))
        .route("/positions/:id", get(api::get_position))
        .route("/positions", get(api::list_positions))
        .route("/positions/:id/close", post(api::close_position))   // <-- NEW CLOSE ROUTE
        .with_state(pool);

    // Run server
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("🚀 Server running at http://{}/health", addr);

    axum::serve(
        tokio::net::TcpListener::bind(addr).await?,
        app,
    )
    .await?;

    Ok(())
}

async fn health_check() -> &'static str {
    "OK"
}
