use dioxus::prelude::*;
use crate::routes::Routes;
use crate::components::{Header, Footer};

#[component]
pub fn Base() -> Element {
    rsx! {
        Header {}
        div {
            class: "container w-auto px-4 pt-20 pb-5",
            Outlet::<Routes> {}
        }
        Footer {}
    }
}

