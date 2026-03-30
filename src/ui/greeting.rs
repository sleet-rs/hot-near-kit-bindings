use crate::near::near_kit_hot_greeting;
use dioxus::prelude::*;
// ===========================================
#[component]
pub fn greeting() -> Element {
    let mut greeting = use_signal(|| String::new());
    let mut message = use_signal(|| String::new());
    let mut error = use_signal(|| Option::<String>::None);
    let mut loading = use_signal(|| false);

    let get_greeting = move |_| {
        wasm_bindgen_futures::spawn_local(async move {
            loading.set(true);
            error.set(None);
            message.set("Loading greeting...".to_string());

            match near_kit_hot_greeting::get_greeting("hello.sleet.testnet").await {
                Ok(result) => {
                    greeting.set(result.clone());
                    message.set(format!("Greeting: {}", result));
                }
                Err(e) => {
                    error.set(Some(format!("Error: {}", e)));
                    message.clear();
                }
            }
            loading.set(false);
        });
    };

    let set_greeting = move |_| {
        let greeting_value = greeting();
        if greeting_value.is_empty() {
            error.set(Some("Error: Greeting cannot be empty".to_string()));
            return;
        }

        wasm_bindgen_futures::spawn_local(async move {
            loading.set(true);
            error.set(None);
            message.set("Setting greeting...".to_string());

            match near_kit_hot_greeting::set_greeting("hello.sleet.testnet", &greeting_value).await {
                Ok(outcome) => {
                    if outcome.is_success() {
                        message.set(format!("✓ Greeting set to: {}", greeting_value));
                    } else if let Some(err_msg) = outcome.failure_message() {
                        error.set(Some(format!("Transaction failed: {}", err_msg)));
                    } else {
                        error.set(Some("Transaction failed with unknown error".to_string()));
                    }
                }
                Err(e) => {
                    error.set(Some(format!("Error: {}", e)));
                    message.clear();
                }
            }
            loading.set(false);
        });
    };

    rsx! {
        div {
            style: "padding: 16px; border: 1px solid #ccc; border-radius: 8px; max-width: 400px;",
            h3 { "Greeting Contract" }

            input {
                r#type: "text",
                value: "{greeting}",
                oninput: move |e| greeting.set(e.value()),
                placeholder: "Enter greeting",
                style: "width: 100%; padding: 8px; margin-bottom: 12px; border: 1px solid #ddd; border-radius: 4px;",
            }

            div { style: "display: flex; gap: 8px; margin-bottom: 12px;",
                button {
                    onclick: get_greeting,
                    disabled: loading(),
                    style: "padding: 8px 16px; background: #007bff; color: white; border: none; border-radius: 4px; cursor: pointer;",
                    if loading() { "Loading..." } else { "GET GREETING" }
                }

                button {
                    onclick: set_greeting,
                    disabled: loading(),
                    style: "padding: 8px 16px; background: #28a745; color: white; border: none; border-radius: 4px; cursor: pointer;",
                    if loading() { "Setting..." } else { "SET GREETING" }
                }
            }

            if let Some(err) = error() {
                div {
                    style: "padding: 8px; background: #f8d7da; color: #721c24; border-radius: 4px; margin-bottom: 8px;",
                    "{err}"
                }
            }

            if !message().is_empty() && error().is_none() {
                div {
                    style: "padding: 8px; background: #d4edda; color: #155724; border-radius: 4px;",
                    "{message}"
                }
            }
        }
    }
}
// ===========================================
