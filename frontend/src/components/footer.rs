use dioxus::prelude::*;

#[component]
pub fn Footer() -> Element {
    rsx! {
        footer {
            p { "© 2024 Cat Cafe" }
        }
    }
}

