use dioxus::prelude::*;
use hot_near_kit_bindings::logic::hello::hello;
use hot_near_kit_bindings::ui::button_login::button_login;
use hot_near_kit_bindings::ui::button_network::button_network;
// ===========================================
const FAVICON: Asset = asset!("/assets/icon.svg");
const MAIN_CSS: Asset = asset!("/assets/main.css");
// ===========================================
fn main() {
    dioxus::launch(App);
}
// ===========================================
#[component]
fn App() -> Element {
    use_effect(|| {
        hello();
    });
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        Hero {}

    }
}
// ===========================================
#[component]
pub fn Hero() -> Element {
    rsx! {
        section {
            h1 { "hot-near-kit-bindings" }
            p { "🦀 rust bindings for a custom near kit and near connect client" }
            div {
                id: "demo_div",
                button_login {}
                br {  }
                button_network {}
            }

            p {
                "copyright 2026 by sleet.near"
            }
        }
    }
}
// ===========================================
