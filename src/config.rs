use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub server_addr: String,
    pub firebase_project_id: String, // Firebase project ID
    pub firebase_api_key: String,    // Firebase API key
}

pub fn load_config() -> Config {
    Config {
        server_addr: "127.0.0.1:3000".to_string(),
        firebase_project_id: "your-firebase-project-id".to_string(),
        firebase_api_key: "your-firebase-api-key".to_string(),
    }
}
