use dioxus::prelude::*;
use crate::routes::Routes;

#[component]
pub fn App() -> Element {
    rsx! {
        document::Meta { charset: "UTF-8" }
        document::Meta { name: "viewport", content: "width=device-width, initial-scale=1.0" }
        document::Link { href: "https://fonts.googleapis.com", rel: "preconnect" }
        document::Link { href: "https://fonts.gstatic.com", rel: "preconnect", crossorigin: "true" }
        document::Stylesheet { href: asset!("/assets/tailwind.css") }
        document::Stylesheet { href: asset!("/assets/main.css") }
        Router::<Routes> {}
    }
}

