use axum::{extract::State, Json};
use crate::{
    error::AppError,
    models::{
        audit::AuditEntry,
        execution::{ExecutionReceipt, ExecutionSubmitRequest},
    },
    services::{oneshot::OneShotService, permission::PermissionService},
    state::AppState,
};
use chrono::Utc;
use uuid::Uuid;

pub async fn submit_execution(
    State(state): State<AppState>,
    Json(req): Json<ExecutionSubmitRequest>,
) -> Result<Json<ExecutionReceipt>, AppError> {
    let permission = PermissionService::get(&state, &req.permission_id).await?;

    if !permission.is_valid() {
        return Err(AppError::PermissionInvalid);
    }

    // Enforce target whitelist
    if !permission.allowed_targets.iter().any(|t| {
        t.to_lowercase() == req.tx_data.to.to_lowercase()
    }) {
        return Err(AppError::PermissionInvalid);
    }

    let receipt =
        OneShotService::relay_transaction(&state, &req.permission_id, &req.tx_data).await?;

    // Append to immutable audit trail
    let entry = AuditEntry {
        entry_id: Uuid::new_v4().to_string(),
        permission_id: req.permission_id.clone(),
        action_performed: permission.intent_summary.clone(),
        tx_hash: Some(receipt.tx_hash.clone()),
        ai_explanation: receipt.ai_explanation.clone(),
        status: format!("{:?}", receipt.status).to_lowercase(),
        timestamp: Utc::now(),
        smart_account: permission.smart_account,
    };

    state.audit_log.write().await.push(entry);

    Ok(Json(receipt))
}