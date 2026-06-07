use axum::{extract::State, Json};
use crate::{
    error::AppError,
    models::ai::{ExecutionPlanResponse, IntentRequest},
    services::venice::VeniceService,
    state::AppState,
};

pub async fn plan_intent(
    State(state): State<AppState>,
    Json(req): Json<IntentRequest>,
) -> Result<Json<ExecutionPlanResponse>, AppError> {
    if req.intent.trim().is_empty() {
        return Err(AppError::InvalidIntent("Intent cannot be empty".into()));
    }
    if req.intent.len() > 500 {
        return Err(AppError::InvalidIntent("Intent exceeds 500 characters".into()));
    }

    let plan = VeniceService::generate_execution_plan(&state, &req.intent).await?;
    Ok(Json(plan))
}