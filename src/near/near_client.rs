use wasm_bindgen::prelude::*;
// ===========================================
#[wasm_bindgen(module = "/assets/near_client.js")]
extern "C" {
    #[wasm_bindgen(js_name = "near_auth_status")]
    pub fn near_auth_status() -> String;
    #[wasm_bindgen(catch)]
    pub async fn near_request_sign_in() -> Result<JsValue, JsValue>;
    #[wasm_bindgen(js_name = "near_account_id")]
    pub fn near_account_id() -> String;
    #[wasm_bindgen(catch)]
    pub async fn near_sign_out() -> Result<JsValue, JsValue>;
}
// ===========================================
