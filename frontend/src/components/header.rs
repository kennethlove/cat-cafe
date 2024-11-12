use dioxus::prelude::*;

#[component]
pub fn Header() -> Element {
    rsx! {
        header {
            h1 { "Cat Cafe" }
        }
    }
}

