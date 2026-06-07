use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct IntentRequest {
    pub intent: String,
    pub wallet_address: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ExecutionStep {
    pub step_number: u8,
    pub action: String,
    pub description: String,
    pub target_contract: Option<String>,
    pub function_selector: Option<String>,
}

#[derive(Debug, Serialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum RiskLevel {
    Low,
    Medium,
    High,
}

#[derive(Debug, Serialize, Clone)]
pub struct ParsedIntent {
    pub action_type: String,   // "swap" | "send" | "approve" | "stake"
    pub asset: Option<String>,
    pub amount: Option<String>,
    pub destination: Option<String>,
    pub duration_seconds: Option<u64>,
}

#[derive(Debug, Serialize)]
pub struct ExecutionPlanResponse {
    pub steps: Vec<ExecutionStep>,
    pub risk: RiskLevel,
    pub parsed_intent: ParsedIntent,
    pub human_explanation: String,
    pub estimated_gas: String,
}