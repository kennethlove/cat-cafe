use dioxus::prelude::*;
use crate::routes::Routes;
use crate::components::{Header, Footer};

#[component]
pub fn Base() -> Element {
    rsx! {
        div {
            id: "layout",
            class: "container mx-auto px-6 py-8 md:px-4 lg:px-8",
            Header {}
            div {
                class: "mt-4",
                Outlet::<Routes> {}
            }
            Footer {}
        }
    }
}

