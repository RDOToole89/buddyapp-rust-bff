use axum::{
    routing::{get, post},
    Router,
};
use tracing_subscriber;

mod config;
mod routes;
mod services;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let config = config::load_config();
    let test_routes = Router::new()
        .route("/test-get", get(test_get_handler))
        .route("/test-post", post(test_post_handler))
        .route("/test-firebase", get(test_firebase_handler)); // Correctly use `test_firebase_handler`
    let app = routes::create_router().merge(test_routes);

    let addr = config.server_addr.parse().unwrap();
    tracing::info!("Server running at http://{}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn test_get_handler() -> &'static str {
    "This is a test GET route!"
}

async fn test_post_handler() -> &'static str {
    "This is a test POST route!"
}

// Handler for testing Firebase connection
async fn test_firebase_handler() -> &'static str {
    let config = config::load_config();
    match services::firebase::test_firebase_connection(&config.firebase_project_id).await {
        Ok(_) => "Firebase connection successful",
        Err(err) => {
            eprintln!("Firebase connection error: {}", err);
            "Failed to connect to Firebase"
        }
    }
}
