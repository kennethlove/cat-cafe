use dioxus::prelude::*;
use crate::routes::Routes;
use crate::components::{Header, Footer};

#[component]
pub fn Base() -> Element {
    rsx! {
        div {
            id: "layout",
            Header {}
            div {
                class: "mt-4",
                Outlet::<Routes> {}
            }
            Footer {}
        }
    }
}

