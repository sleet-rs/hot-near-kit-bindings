use crate::near::near_kit_hot_connect::{
    near_account_id, near_auth_status, near_request_sign_in, near_sign_out,
};
use dioxus::prelude::*;
// ===========================================
#[component]
pub fn button_login() -> Element {
    let mut is_signed_in = use_signal(|| false);
    let mut account_id = use_signal(|| Option::<String>::None);

    use_effect(move || {
        wasm_bindgen_futures::spawn_local(async move {
            if let Ok(status) = near_auth_status().await {
                let status_str = status.as_string().unwrap_or_default();
                let signed_in = status_str == "SignedIn";
                is_signed_in.set(signed_in);
                if signed_in {
                    if let Ok(acc_id) = near_account_id().await {
                        account_id.set(Some(acc_id.as_string().unwrap_or_default()));
                    }
                } else {
                    account_id.set(None);
                }
            }
        });
    });

    let on_login = move |_| {
        wasm_bindgen_futures::spawn_local(async move {
            if near_request_sign_in().await.is_ok() {
                if let Ok(status) = near_auth_status().await {
                    let status_str = status.as_string().unwrap_or_default();
                    let signed_in = status_str == "SignedIn";
                    is_signed_in.set(signed_in);
                    if signed_in {
                        if let Ok(acc_id) = near_account_id().await {
                            account_id.set(Some(acc_id.as_string().unwrap_or_default()));
                        }
                    } else {
                        account_id.set(None);
                    }
                }
            }
        });
    };

    let on_logout = move |_| {
        wasm_bindgen_futures::spawn_local(async move {
            if near_sign_out().await.is_ok() {
                is_signed_in.set(false);
                account_id.set(None);
            }
        });
    };

    rsx! {
        if is_signed_in() {
            button { onclick: on_logout,
                "LOGOUT ",
                {account_id().as_deref().unwrap_or("")}
            }
        } else {
            button { onclick: on_login,
                "LOGIN"
            }
        }
    }
}
// ===========================================
