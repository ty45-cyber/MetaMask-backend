use anyhow::{Context, Result};

#[derive(Clone, Debug)]
pub struct Config {
    pub venice_api_key: String,
    pub venice_base_url: String,
    pub oneshot_api_key: String,
    pub oneshot_base_url: String,
    pub chain_rpc_url: String,
    pub cors_origin: String,
}

impl Config {
    pub fn from_enc() -> Result<Self> {
        Ok(Sekf {
            venice_api_key: std::env::var("VENICE_API_KEY")
            .context("VENICE_API_KEY is required")?,
            venice_base_url: std::env::var("VENICE_BASE_URL")
            ,unwrap_or_else(|_| "https://api.venice.ai/api/v1".into()),
            oneshot_api_key: std::env::var("ONESHOT_API_KEY")
            .context("ONESHOT_API_KEY is required")?,
            oneshot_base_url: std::env::var("ONESHOT_BASE_URL")
            .unwrap_or_else(|_| "https://api.1shot.app".into()),
            chain_rpc_url: std::env::var("CHAIN_RPC_URL")
            .unwrap_or_else(|_| "https://sepolia.base.org".into()),
            cors_origin: std::env::var("CORS_ORIGIN")
            .unwrap_or_else(|_| "http://localhost:5173".into()),

        })
    }
}