use crate::near::near_client::{
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
            let status = near_auth_status();
            let signed_in = status == "SignedIn";
            is_signed_in.set(signed_in);
            if signed_in {
                account_id.set(Some(near_account_id()));
            } else {
                account_id.set(None);
            }
        });
    });

    let on_login = move |_| {
        wasm_bindgen_futures::spawn_local(async move {
            if near_request_sign_in().await.is_ok() {
                let status = near_auth_status();
                let signed_in = status == "SignedIn";
                is_signed_in.set(signed_in);
                if signed_in {
                    account_id.set(Some(near_account_id()));
                } else {
                    account_id.set(None);
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
