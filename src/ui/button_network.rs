use dioxus::prelude::*;
use web_sys::console;
// ===========================================
const NETWORK_STORAGE_KEY: &str = "network_id";
// ===========================================
#[component]
pub fn button_network() -> Element {
    let mut network_id = use_signal(|| "mainnet".to_string());

    use_effect(move || {
        if let Some(storage) = web_sys::window().unwrap().local_storage().unwrap() {
            if let Ok(Some(saved)) = storage.get_item(NETWORK_STORAGE_KEY) {
                network_id.set(saved);
            }
        }
    });

    let toggle_network = move |_| {
        let new_network = if network_id() == "mainnet" {
            "testnet"
        } else {
            "mainnet"
        };
        network_id.set(new_network.to_string());

        if let Some(storage) = web_sys::window().unwrap().local_storage().unwrap() {
            let _ = storage.set_item(NETWORK_STORAGE_KEY, new_network);
        }

        console::log_1(&format!("{}", new_network).into());
    };

    rsx! {
        button {
            onclick: toggle_network,
            "{network_id().to_uppercase()}"
        }
    }
}
// ===========================================
