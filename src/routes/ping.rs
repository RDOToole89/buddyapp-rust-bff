use axum::{routing::get, Router};

/// This function returns a router for the ping route
pub fn routes() -> Router {
    Router::new().route("/", get(ping_handler))
}

/// The handler for the `/ping` endpoint
async fn ping_handler() -> &'static str {
    "Pong!"
}
