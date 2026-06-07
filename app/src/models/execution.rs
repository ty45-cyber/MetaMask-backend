use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct ExecutionSubmitRequest {
    pub permission_id: String,
    pub tx_data: TransactionData,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TransactionData {
    pub to: String,
    pub data: String,         // hex-encoded calldata
    pub value: String,        // wei as string
    pub chain_id: u64,
}

#[derive(Debug, Serialize)]
pub struct ExecutionReceipt {
    pub receipt_id: String,
    pub permission_id: String,
    pub tx_hash: String,
    pub status: ExecutionStatus,
    pub gas_used: Option<String>,
    pub block_number: Option<u64>,
    pub timestamp: DateTime<Utc>,
    pub ai_explanation: String,
    pub relay_provider: String,
}

#[derive(Debug, Serialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ExecutionStatus {
    Pending,
    Confirmed,
    Failed,
}