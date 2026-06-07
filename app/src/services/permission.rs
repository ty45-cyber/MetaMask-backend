use crate::{
    error::AppError,
    models::permission::{
        ContractMapping, PermissionCreateRequest, PermissionPreviewRequest,
        PermissionPreviewResponse, ScopeSummary, StoredPermission,
    },
    state::AppState,
};

pub struct PermissionService;

impl PermissionService {
    pub async fn preview(
        state: &AppState,
        req: &PermissionPreviewRequest,
    ) -> Result<PermissionPreviewResponse, AppError> {
        // Deterministic preview based on parsed intent keywords
        let intent_lower = req.intent.to_lowercase();

        let (action, asset, max_amount, risk_level, recommended_expiry, mappings) =
            Self::classify_intent(&intent_lower);

        Ok(PermissionPreviewResponse {
            scope_summary: ScopeSummary {
                action,
                asset,
                max_amount,
                time_limit: format!("{} seconds", recommended_expiry),
            },
            risk_level,
            contract_mappings: mappings,
            recommended_expiry,
        })
    }

    pub async fn create(
        state: &AppState,
        req: PermissionCreateRequest,
    ) -> Result<StoredPermission, AppError> {
        if req.allowed_targets.is_empty() {
            return Err(AppError::InvalidIntent(
                "At least one allowed target is required".into(),
            ));
        }
        if req.expiry_seconds == 0 || req.expiry_seconds > 86_400 * 30 {
            return Err(AppError::InvalidIntent(
                "Expiry must be between 1 second and 30 days".into(),
            ));
        }

        let permission = StoredPermission::new(req);
        let mut store = state.permissions.write().await;
        store.insert(permission.permission_id.clone(), permission.clone());

        Ok(permission)
    }

    pub async fn get(
        state: &AppState,
        permission_id: &str,
    ) -> Result<StoredPermission, AppError> {
        let store = state.permissions.read().await;
        store
            .get(permission_id)
            .cloned()
            .ok_or_else(|| AppError::PermissionNotFound(permission_id.to_string()))
    }

    pub async fn revoke(state: &AppState, permission_id: &str) -> Result<(), AppError> {
        let mut store = state.permissions.write().await;
        let perm = store
            .get_mut(permission_id)
            .ok_or_else(|| AppError::PermissionNotFound(permission_id.to_string()))?;
        perm.is_revoked = true;
        Ok(())
    }

    fn classify_intent(
        intent: &str,
    ) -> (String, String, String, String, u64, Vec<ContractMapping>) {
        if intent.contains("swap") {
            (
                "Token Swap".into(),
                Self::extract_asset(intent),
                "100 USDC".into(),
                "medium".into(),
                600,
                vec![ContractMapping {
                    contract_address: "0x7a250d5630B4cF539739dF2C5dAcb4c659F2488D".into(),
                    contract_name: "Uniswap V2 Router".into(),
                    function_name: "swapExactTokensForTokens".into(),
                    selector: "0x38ed1739".into(),
                }],
            )
        } else if intent.contains("send") || intent.contains("transfer") {
            (
                "Token Transfer".into(),
                Self::extract_asset(intent),
                "50 USDC".into(),
                "low".into(),
                3600,
                vec![ContractMapping {
                    contract_address: "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48".into(),
                    contract_name: "USDC Token".into(),
                    function_name: "transfer".into(),
                    selector: "0xa9059cbb".into(),
                }],
            )
        } else {
            (
                "Generic Execution".into(),
                "ETH".into(),
                "0.01 ETH".into(),
                "medium".into(),
                1800,
                vec![],
            )
        }
    }

    fn extract_asset(intent: &str) -> String {
        for token in &["usdc", "eth", "dai", "usdt", "weth", "wbtc"] {
            if intent.contains(token) {
                return token.to_uppercase();
            }
        }
        "USDC".into()
    }
}