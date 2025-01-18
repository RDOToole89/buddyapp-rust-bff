use axum::{routing::post, Json, Router};
use serde::{Deserialize, Serialize};
#[derive(Debug, Deserialize)]
struct AuthRequest {
    id_token: String,
}

#[derive(Debug, Serialize)]
struct AuthResponse {
    user_id: Option<String>,
    email: Option<String>,
    is_valid: bool,
}

pub fn routes() -> Router {
    Router::new().route("/verify", post(verify_token))
}

async fn verify_token(Json(payload): Json<AuthRequest>) -> Json<AuthResponse> {
    // Call a service function to verify the Firebase token
    match crate::services::auth::verify_token(&payload.id_token).await {
        Ok(user) => Json(AuthResponse {
            user_id: Some(user.user_id),
            email: user.email,
            is_valid: true,
        }),
        Err(_) => Json(AuthResponse {
            user_id: None,
            email: None,
            is_valid: false,
        }),
    }
}
