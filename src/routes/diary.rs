use axum::{routing::post, Json, Router};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct DiaryEntryRequest {
    user_id: String,
    entry: String,
    timestamp: String,
}

#[derive(Debug, Serialize)]
pub struct DiaryEntryResponse {
    status: String,
    entry_id: String,
}

pub fn routes() -> Router {
    Router::new()
        .route("/add", post(add_entry))
        .route("/entries", post(get_entries))
}

async fn add_entry(Json(payload): Json<DiaryEntryRequest>) -> Json<DiaryEntryResponse> {
    // Mock implementation
    Json(DiaryEntryResponse {
        status: "Entry added".to_string(),
        entry_id: "entry_12345".to_string(),
    })
}

async fn get_entries() -> &'static str {
    "Diary entries endpoint is under development."
}
