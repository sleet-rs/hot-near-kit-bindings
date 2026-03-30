use wasm_bindgen::prelude::*;
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
