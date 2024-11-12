use dioxus::prelude::*;
use crate::routes::Routes;
#[component]
pub fn Cats() -> Element {
    rsx! {
        div {
            h1 { "Cats" }
            Outlet::<Routes> {}
        }
    }
}

