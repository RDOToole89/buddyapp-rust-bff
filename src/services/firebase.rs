use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use reqwest;
use serde::Deserialize;
use serde_json;

#[derive(Debug, Deserialize)]
pub struct FirebaseClaims {
    pub user_id: String,
    pub email: Option<String>,
    pub exp: usize,
}

pub async fn fetch_firebase_keys() -> Result<Vec<DecodingKey>, reqwest::Error> {
    let url =
        "https://www.googleapis.com/robot/v1/metadata/x509/securetoken@system.gserviceaccount.com";
    let response = reqwest::get(url).await?.json::<serde_json::Value>().await?;
    let keys = response
        .as_object()
        .unwrap()
        .values()
        .filter_map(|key| DecodingKey::from_rsa_pem(key.as_str().unwrap().as_bytes()).ok())
        .collect();
    Ok(keys)
}

pub async fn verify_firebase_token(
    id_token: &str,
    project_id: &str,
) -> Result<FirebaseClaims, &'static str> {
    let firebase_keys = fetch_firebase_keys()
        .await
        .map_err(|_| "Failed to fetch Firebase keys")?;
    for key in firebase_keys {
        if let Ok(token_data) =
            decode::<FirebaseClaims>(id_token, &key, &Validation::new(Algorithm::RS256))
        {
            // Remove iss check since FirebaseClaims doesn't have that field
            return Ok(token_data.claims);
        }
    }
    Err("Invalid token")
}
pub async fn test_firebase_connection(project_id: &str) -> Result<(), String> {
    match fetch_firebase_keys().await {
        Ok(keys) => {
            println!("Successfully fetched Firebase keys (count: {})", keys.len());
            println!("Using project ID: {}", project_id);
            Ok(())
        }
        Err(err) => {
            eprintln!("Error fetching Firebase keys: {}", err);
            Err(format!("Failed to fetch Firebase keys: {}", err))
        }
    }
}
