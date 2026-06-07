use axum::{
    extract::{Query, State},
    Json,
};
use serde::Deserialize;
use crate::{error::AppError, models::audit::AuditEntry, state::AppState};

#[derive(Debug, Deserialize)]
pub struct AuditQuery {
    pub smart_account: Option<String>,
    pub limit: Option<usize>,
}

pub async fn get_activity(
    State(state): State<AppState>,
    Query(q): Query<AuditQuery>,
) -> Result<Json<Vec<AuditEntry>>, AppError> {
    let log = state.audit_log.read().await;
    let limit = q.limit.unwrap_or(50).min(200);

    let entries: Vec<AuditEntry> = log
        .iter()
        .rev() // newest first
        .filter(|e| {
            q.smart_account
                .as_ref()
                .map(|addr| e.smart_account.to_lowercase() == addr.to_lowercase())
                .unwrap_or(true)
        })
        .take(limit)
        .cloned()
        .collect();

    Ok(Json(entries))
}