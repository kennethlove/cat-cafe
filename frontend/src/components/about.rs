use dioxus::prelude::*;

#[component]
pub fn About() -> Element {
    rsx! {
        div {
            h1 { "About" }
            p { "Dioxus is a full-stack web framework for Rust and WebAssembly." }
        }
    }
}

