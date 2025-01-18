use axum::{
    routing::{get, post},
    Router,
}; // Router for defining routes, post for POST routes
use tracing_subscriber; // For logging and runtime information

mod config; // Handles configuration settings (e.g., server address)
mod routes; // Manages and combines application routes
mod services; // Contains business logic like Firebase authentication

#[tokio::main] // Sets up the async runtime using Tokio
async fn main() {
    tracing_subscriber::fmt::init(); // Initialize structured logging

    let config = config::load_config(); // Load configuration values
    let test_routes = Router::new() // Add test routes for GET and POST
        .route("/test-get", get(test_get_handler))
        .route("/test-post", post(test_post_handler));

    let app = routes::create_router().merge(test_routes); // Combine test routes with app routes
    let addr = config.server_addr.parse().unwrap(); // Parse server address
    tracing::info!("Server running at http://{}", addr); // Log server address

    axum::Server::bind(&addr) // Bind and serve requests
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn test_get_handler() -> &'static str {
    "This is a test GET route!" // Response for test GET
}

async fn test_post_handler() -> &'static str {
    "This is a test POST route!" // Response for test POST
}
