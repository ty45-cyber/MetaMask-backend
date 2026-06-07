use std::sync::Arc;
use tokio::sync::RwLock;
use std::collections::HashMap;
use crate::{config::Config, models::permission::StoredPermission, models::audit::AuditEntry};

#[derive(Clone)]
pub struct AppState {
    pub config: Config,
    pub http: reqwest::Client,
    // In-memory stores for hackathon scope — swap for DB in production
    pub permissions: Arc<RwLock<HashMap<String, StoredPermission>>>,
    pub audit_log: Arc<RwLock<Vec<AuditEntry>>>,
}

impl AppState {
    pub async fn new(config: Config) -> anyhow::Result<Self> {
        let http = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()?;

        Ok(Self {
            config,
            http,
            permissions: Arc::new(RwLock::new(HashMap::new())),
            audit_log: Arc::new(RwLock::new(Vec::new())),
        })
    }
}