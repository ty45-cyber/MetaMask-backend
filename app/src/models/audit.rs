use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
pub struct AuditEntry {
    pub entry_id: String,
    pub permission_id: String,
    pub action_performed: String,
    pub tx_hash: Option<String>,
    pub ai_explanation: String,
    pub status: String,
    pub timestamp: DateTime<Utc>,
    pub smart_account: String,
}