use dioxus::prelude::*;
use crate::routes::Routes;

#[component]
pub fn App() -> Element {
    rsx! {
        Router::<Routes> {}
    }
}

