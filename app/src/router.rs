use axum::{
    routing::{get, post, delete},
    Router,
};
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use crate::{
    handlers::{ai, audit, execution, permission},
    state::AppState,
};

pub fn build(state: AppState) -> Router {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    Router::new()
        // AI planning
        .route("/api/ai/plan", post(ai::plan_intent))
        // Permissions
        .route("/api/permission/preview", post(permission::preview_permission))
        .route("/api/permission/create", post(permission::create_permission))
        .route("/api/permission/:id", get(permission::get_permission))
        .route("/api/permission/:id/revoke", delete(permission::revoke_permission))
        // Execution
        .route("/api/execution/submit", post(execution::submit_execution))
        // Audit
        .route("/api/activity", get(audit::get_activity))
        // Health
        .route("/health", get(|| async { "Mission Control nominal." }))
        .layer(TraceLayer::new_for_http())
        .layer(cors)
        .with_state(state)
}