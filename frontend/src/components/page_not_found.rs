use dioxus::prelude::*;

#[component]
pub fn PageNotFound(route: Vec<String>) -> Element {
    rsx! {
        div {
            h1 { "404" }
            p { "Page not found: {route:?}" }
        }
    }
}

