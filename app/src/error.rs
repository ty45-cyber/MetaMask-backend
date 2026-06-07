use axum::{http::StatusCode, response::{IntoResponse, Response}, Json};
use serde_json::json;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Venice AI error: {0}")]
    VeniceError(String),
    #[error("1Shot relayer error: {0}")]
    OneShotError(String),
    #[error("Permission not found: {0}")]
    PermissionNotFound(String),
    #[error("Permission expired or revoked")]
    PermissionInvalid,
    #[error("Execution limit exceeded")]
    LimitExceeded,
    #[error("Invalid intent: {0}")]
    InvalidIntent(String),
    #[error("Internal error: {0}")]
    Internal(#[from] anyhow::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            AppError::PermissionNotFound(_) => (StatusCode::NOT_FOUND, self.to_string()),
            AppError::PermissionInvalid | AppError::LimitExceeded => {
                (StatusCode::FORBIDDEN, self.to_string())
            }
            AppError::InvalidIntent(_) => (StatusCode::BAD_REQUEST, self.to_string()),
            AppError::VeniceError(_) | AppError::OneShotError(_) => {
                (StatusCode::BAD_GATEWAY, self.to_string())
            }
            AppError::Internal(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal server error".to_string(),
            ),
        };

        let body = Json(json!({ "error": message }));
        (status, body).into_response()
    }
}