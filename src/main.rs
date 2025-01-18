use axum::{routing::post, Router};
use tracing_subscriber;

mod config;
mod routes;
mod services;

#[tokio::main]
async fn main() {
    // Initialize logging
    tracing_subscriber::fmt::init();

    // Load configuration
    let config = config::load_config();

    // Build the application router
    let app = routes::create_router();

    // Start the server
    let addr = config.server_addr.parse().unwrap();
    tracing::info!("Server running at http://{}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
