use axum::{routing::get, Json, Router};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct ChatRequest {
    sender_id: String,
    receiver_id: String,
    message: String,
}

#[derive(Debug, Serialize)]
pub struct ChatResponse {
    status: String,
    timestamp: String,
}

pub fn routes() -> Router {
    Router::new()
        .route("/send", get(send_message))
        .route("/history", get(get_chat_history))
}

async fn send_message(Json(payload): Json<ChatRequest>) -> Json<ChatResponse> {
    // Mock implementation
    Json(ChatResponse {
        status: "Message sent".to_string(),
        timestamp: "2025-01-18T17:00:00Z".to_string(),
    })
}

async fn get_chat_history() -> &'static str {
    "Chat history endpoint is under development."
}
