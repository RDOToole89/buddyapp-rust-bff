use axum::routing::get;
use axum::Router;

pub mod auth;
pub mod chat;
pub mod diary;
pub mod ping;

pub fn create_router() -> Router {
    Router::new()
        .route("/ping", get(ping_handler))
        .nest("/auth", auth::routes())
        .nest("/chat", chat::routes())
        .nest("/diary", diary::routes())
}

async fn ping_handler() -> &'static str {
    "Pong!"
}
