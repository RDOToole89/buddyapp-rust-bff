use dotenvy::dotenv;
use serde::Deserialize;
use std::env;

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct Config {
    pub server_addr: String,
    pub firebase_project_id: String,
    pub firebase_api_key: String,
}

pub fn load_config() -> Config {
    dotenv().ok(); // Load environment variables from `.env`

    Config {
        server_addr: env::var("SERVER_ADDR").unwrap_or_else(|_| "127.0.0.1:3000".to_string()),
        firebase_project_id: env::var("FIREBASE_PROJECT_ID")
            .expect("FIREBASE_PROJECT_ID must be set"),
        firebase_api_key: env::var("FIREBASE_API_KEY").expect("FIREBASE_API_KEY must be set"),
    }
}
