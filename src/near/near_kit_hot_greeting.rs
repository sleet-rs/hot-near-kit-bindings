use wasm_bindgen::prelude::*;
use serde_json::Value;
use crate::near::types::FinalExecutionOutcome;
// ===========================================
#[wasm_bindgen(module = "/assets/js/near_kit_hot_greeting.js")]
extern "C" {
    #[wasm_bindgen(catch)]
    pub async fn greeting_get_greeting(
        greeting_contractId: &str,
    ) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch)]
    pub async fn greeting_set_greeting(
        greeting_contractId: &str,
        greeting: &str,
    ) -> Result<JsValue, JsValue>;
}
// ===========================================

/// Helper to parse greeting_get_greeting result as String
pub async fn get_greeting(greeting_contract_id: &str) -> Result<String, String> {
    let result = greeting_get_greeting(greeting_contract_id)
        .await
        .map_err(|e| format!("{:?}", e))?;
    
    Ok(result.as_string().unwrap_or_default())
}

/// Helper to parse greeting_set_greeting result as FinalExecutionOutcome
pub async fn set_greeting(
    greeting_contract_id: &str,
    greeting: &str,
) -> Result<FinalExecutionOutcome, String> {
    let result = greeting_set_greeting(greeting_contract_id, greeting)
        .await
        .map_err(|e| format!("{:?}", e))?;
    
    // Convert JsValue to serde_json::Value, then deserialize to FinalExecutionOutcome
    let json_value: Value = serde_wasm_bindgen::from_value(result)
        .map_err(|e| format!("Failed to parse result: {:?}", e))?;
    
    let outcome: FinalExecutionOutcome = serde_json::from_value(json_value)
        .map_err(|e| format!("Failed to deserialize FinalExecutionOutcome: {:?}", e))?;
    
    Ok(outcome)
}
// ===========================================
