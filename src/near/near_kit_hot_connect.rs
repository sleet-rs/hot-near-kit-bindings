use wasm_bindgen::prelude::*;
// ===========================================
#[wasm_bindgen(module = "/assets/near_kit_hot_connect.js")]
extern "C" {
    #[wasm_bindgen(catch)]
    pub async fn near_auth_status() -> Result<JsValue, JsValue>;
    #[wasm_bindgen(catch)]
    pub async fn near_request_sign_in() -> Result<JsValue, JsValue>;
    #[wasm_bindgen(catch)]
    pub async fn near_account_id() -> Result<JsValue, JsValue>;
    #[wasm_bindgen(catch)]
    pub async fn near_sign_out() -> Result<JsValue, JsValue>;
}
// ===========================================
