use crate::services::firebase;
use axum::{routing::post, Json, Router};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct AuthRequest {
    pub id_token: String, // ID token to be verified
}

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub user_id: Option<String>, // Firebase user ID
    pub email: Option<String>,   // User's email
    pub is_valid: bool,          // Whether the token is valid
}

pub fn routes() -> Router {
    Router::new().route("/verify", post(verify_token)) // Define the /verify route
}

async fn verify_token(Json(payload): Json<AuthRequest>) -> Json<AuthResponse> {
    let config = crate::config::load_config(); // Load project configuration
    match firebase::verify_firebase_token(&payload.id_token, &config.firebase_project_id).await {
        Ok(claims) => Json(AuthResponse {
            user_id: Some(claims.user_id), // Extract user ID from claims
            email: claims.email,           // Extract email from claims (optional)
            is_valid: true,                // Mark as valid
        }),
        Err(_) => Json(AuthResponse {
            user_id: None,   // No user ID for invalid token
            email: None,     // No email for invalid token
            is_valid: false, // Mark as invalid
        }),
    }
}
