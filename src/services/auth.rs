use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use reqwest;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct FirebaseClaims {
    pub user_id: String,
    pub email: Option<String>,
    pub exp: usize,
}

pub async fn verify_token(id_token: &str) -> Result<FirebaseClaims, &'static str> {
    let firebase_keys = fetch_firebase_keys()
        .await
        .map_err(|_| "Failed to fetch Firebase keys")?;
    for key in firebase_keys {
        if let Ok(token_data) =
            decode::<FirebaseClaims>(id_token, &key, &Validation::new(Algorithm::RS256))
        {
            return Ok(token_data.claims);
        }
    }
    Err("Invalid token")
}

async fn fetch_firebase_keys() -> Result<Vec<DecodingKey>, reqwest::Error> {
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
