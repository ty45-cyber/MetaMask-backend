use axum::{
    extract::{Path, State},
    Json,
};
use crate::{
    error::AppError,
    models::permission::{
        PermissionCreateRequest, PermissionPreviewRequest, PermissionPreviewResponse,
        StoredPermission,
    },
    services::permission::PermissionService,
    state::AppState,
};

pub async fn preview_permission(
    State(state): State<AppState>,
    Json(req): Json<PermissionPreviewRequest>,
) -> Result<Json<PermissionPreviewResponse>, AppError> {
    let preview = PermissionService::preview(&state, &req).await?;
    Ok(Json(preview))
}

pub async fn create_permission(
    State(state): State<AppState>,
    Json(req): Json<PermissionCreateRequest>,
) -> Result<Json<StoredPermission>, AppError> {
    let permission = PermissionService::create(&state, req).await?;
    Ok(Json(permission))
}

pub async fn get_permission(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<StoredPermission>, AppError> {
    let perm = PermissionService::get(&state, &id).await?;
    Ok(Json(perm))
}

pub async fn revoke_permission(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<serde_json::Value>, AppError> {
    PermissionService::revoke(&state, &id).await?;
    Ok(Json(serde_json::json!({ "revoked": true, "permission_id": id })))
}