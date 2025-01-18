use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub server_addr: String,
}

pub fn load_config() -> Config {
    Config {
        server_addr: "127.0.0.1:3000".to_string(),
    }
}
