use crate::{
    error::AppError,
    models::execution::{ExecutionReceipt, ExecutionStatus, TransactionData},
    state::AppState,
};
use chrono::Utc;
use serde_json::{json, Value};
use uuid::Uuid;

pub struct OneShotService;

impl OneShotService {
    pub async fn relay_transaction(
        state: &AppState,
        permission_id: &str,
        tx: &TransactionData,
    ) -> Result<ExecutionReceipt, AppError> {
        let body = json!({
            "to": tx.to,
            "data": tx.data,
            "value": tx.value,
            "chainId": tx.chain_id,
            "permissionId": permission_id
        });

        let response = state
            .http
            .post(format!("{}/v1/relay", state.config.oneshot_base_url))
            .header("x-api-key", &state.config.oneshot_api_key)
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await
            .map_err(|e| AppError::OneShotError(e.to_string()))?;

        if !response.status().is_success() {
            let text = response.text().await.unwrap_or_default();
            return Err(AppError::OneShotError(format!("1Shot error: {}", text)));
        }

        let relay: Value = response
            .json()
            .await
            .map_err(|e| AppError::OneShotError(e.to_string()))?;

        let tx_hash = relay["txHash"]
            .as_str()
            .unwrap_or("0x0000000000000000000000000000000000000000000000000000000000000000")
            .to_string();

        let status = match relay["status"].as_str().unwrap_or("pending") {
            "confirmed" | "success" => ExecutionStatus::Confirmed,
            "failed" => ExecutionStatus::Failed,
            _ => ExecutionStatus::Pending,
        };

        Ok(ExecutionReceipt {
            receipt_id: Uuid::new_v4().to_string(),
            permission_id: permission_id.to_string(),
            tx_hash,
            status,
            gas_used: relay["gasUsed"].as_str().map(|s| s.to_string()),
            block_number: relay["blockNumber"].as_u64(),
            timestamp: Utc::now(),
            ai_explanation: format!(
                "Transaction relayed gaslessly via 1Shot for permission {}.",
                &permission_id[..8]
            ),
            relay_provider: "1Shot".to_string(),
        })
    }
}