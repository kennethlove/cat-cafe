use dioxus::prelude::*;
#[component]
pub fn Home() -> Element {
    rsx! {
        p { "Welcome to the Cat Cafe!" }
    }
}

