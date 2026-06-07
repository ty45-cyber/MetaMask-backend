use crate::{
    error::AppError,
    models::ai::{ExecutionPlanResponse, ExecutionStep, ParsedIntent, RiskLevel},
    state::AppState,
};
use serde_json::{json, Value};

pub struct VeniceService;

impl VeniceService {
    pub async fn generate_execution_plan(
        state: &AppState,
        intent: &str,
    ) -> Result<ExecutionPlanResponse, AppError> {
        let system_prompt = r#"You are a Web3 execution planner for smart account wallets.
Convert user intents into structured JSON execution plans.
Always respond with ONLY valid JSON matching this exact schema:
{
  "steps": [{"step_number": 1, "action": "string", "description": "string", "target_contract": "string|null", "function_selector": "string|null"}],
  "risk": "low"|"medium"|"high",
  "parsed_intent": {
    "action_type": "swap"|"send"|"approve"|"stake",
    "asset": "string|null",
    "amount": "string|null",
    "destination": "string|null",
    "duration_seconds": number|null
  },
  "human_explanation": "one sentence explanation for non-technical user",
  "estimated_gas": "string"
}
Assess risk as low for simple sends, medium for swaps, high for contract approvals or large values."#;

        let body = json!({
            "model": "llama-3.3-70b",
            "messages": [
                {"role": "system", "content": system_prompt},
                {"role": "user", "content": intent}
            ],
            "temperature": 0.1,
            "max_tokens": 800
        });

        let response = state
            .http
            .post(format!("{}/chat/completions", state.config.venice_base_url))
            .header("Authorization", format!("Bearer {}", state.config.venice_api_key))
            .json(&body)
            .send()
            .await
            .map_err(|e| AppError::VeniceError(e.to_string()))?;

        if !response.status().is_success() {
            let text = response.text().await.unwrap_or_default();
            return Err(AppError::VeniceError(format!("Venice API error: {}", text)));
        }

        let raw: Value = response
            .json()
            .await
            .map_err(|e| AppError::VeniceError(e.to_string()))?;

        let content = raw["choices"][0]["message"]["content"]
            .as_str()
            .ok_or_else(|| AppError::VeniceError("Empty Venice response".into()))?;

        // Strip markdown fences if present
        let clean = content
            .trim()
            .trim_start_matches("```json")
            .trim_start_matches("```")
            .trim_end_matches("```")
            .trim();

        let parsed: Value = serde_json::from_str(clean)
            .map_err(|e| AppError::VeniceError(format!("JSON parse error: {}", e)))?;

        Self::map_to_plan(parsed)
    }

    fn map_to_plan(v: Value) -> Result<ExecutionPlanResponse, AppError> {
        let steps: Vec<ExecutionStep> = serde_json::from_value(v["steps"].clone())
            .map_err(|e| AppError::VeniceError(format!("Steps parse error: {}", e)))?;

        let risk = match v["risk"].as_str().unwrap_or("medium") {
            "low" => RiskLevel::Low,
            "high" => RiskLevel::High,
            _ => RiskLevel::Medium,
        };

        let parsed_intent: ParsedIntent =
            serde_json::from_value(v["parsed_intent"].clone())
                .map_err(|e| AppError::VeniceError(format!("Intent parse error: {}", e)))?;

        Ok(ExecutionPlanResponse {
            steps,
            risk,
            parsed_intent,
            human_explanation: v["human_explanation"]
                .as_str()
                .unwrap_or("Executing your request on-chain.")
                .to_string(),
            estimated_gas: v["estimated_gas"]
                .as_str()
                .unwrap_or("~0.001 ETH")
                .to_string(),
        })
    }
}