use crate::logic::contract_hello::hello_contract_id_for_network;
use crate::near::near_kit_hot_greeting;
use dioxus::prelude::*;
// ===========================================
#[component]
pub fn Greeting() -> Element {
    let mut greeting = use_signal(|| String::new());
    let mut message = use_signal(|| String::new());
    let mut error = use_signal(|| Option::<String>::None);
    let mut loading = use_signal(|| false);

    let get_greeting = move |_| {
        wasm_bindgen_futures::spawn_local(async move {
            loading.set(true);
            error.set(None);
            message.set("Loading greeting...".to_string());

            let contract_id = hello_contract_id_for_network();
            match near_kit_hot_greeting::get_greeting(contract_id).await {
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

            let contract_id = hello_contract_id_for_network();
            match near_kit_hot_greeting::set_greeting(contract_id, &greeting_value).await {
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
        div { class: "greeting-container",
            h3 { "Greeting Contract" }

            input {
                r#type: "text",
                value: "{greeting}",
                oninput: move |e| greeting.set(e.value()),
                placeholder: "Enter greeting",
                class: "greeting-input",
            }

            div { class: "greeting-buttons",
                button {
                    onclick: get_greeting,
                    disabled: loading(),
                    class: "greeting-button get",
                    if loading() { "Loading..." } else { "GET GREETING" }
                }

                button {
                    onclick: set_greeting,
                    disabled: loading(),
                    class: "greeting-button set",
                    if loading() { "Setting..." } else { "SET GREETING" }
                }
            }

            if let Some(err) = error() {
                div { class: "greeting-error",
                    "{err}"
                }
            }

            if !message().is_empty() && error().is_none() {
                div { class: "greeting-message",
                    "{message}"
                }
            }
        }
    }
}
// ===========================================
