use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct PermissionPreviewRequest {
    pub intent: String,
    pub smart_account: String,
}

#[derive(Debug, Deserialize)]
pub struct PermissionCreateRequest {
    pub smart_account: String,
    pub agent_address: String,
    pub allowed_targets: Vec<String>,
    pub function_selectors: Vec<String>,
    pub max_value_wei: String,
    pub expiry_seconds: u64,
    pub intent_summary: String,
}

#[derive(Debug, Serialize, Clone)]
pub struct StoredPermission {
    pub permission_id: String,
    pub smart_account: String,
    pub agent_address: String,
    pub allowed_targets: Vec<String>,
    pub function_selectors: Vec<String>,
    pub max_value_wei: String,
    pub expires_at: DateTime<Utc>,
    pub intent_summary: String,
    pub is_revoked: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct PermissionPreviewResponse {
    pub scope_summary: ScopeSummary,
    pub risk_level: String,
    pub contract_mappings: Vec<ContractMapping>,
    pub recommended_expiry: u64,
}

#[derive(Debug, Serialize)]
pub struct ScopeSummary {
    pub action: String,
    pub asset: String,
    pub max_amount: String,
    pub time_limit: String,
}

#[derive(Debug, Serialize)]
pub struct ContractMapping {
    pub contract_address: String,
    pub contract_name: String,
    pub function_name: String,
    pub selector: String,
}

impl StoredPermission {
    pub fn new(req: PermissionCreateRequest) -> Self {
        let expires_at = Utc::now() + chrono::Duration::seconds(req.expiry_seconds as i64);
        Self {
            permission_id: Uuid::new_v4().to_string(),
            smart_account: req.smart_account,
            agent_address: req.agent_address,
            allowed_targets: req.allowed_targets,
            function_selectors: req.function_selectors,
            max_value_wei: req.max_value_wei,
            expires_at,
            intent_summary: req.intent_summary,
            is_revoked: false,
            created_at: Utc::now(),
        }
    }

    pub fn is_valid(&self) -> bool {
        !self.is_revoked && Utc::now() < self.expires_at
    }
}