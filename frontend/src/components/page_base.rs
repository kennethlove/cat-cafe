use dioxus::prelude::*;
use crate::components::{Header, Footer};
use crate::routes::Routes;

#[component]
pub fn PageBase() -> Element {
    rsx! {
        div {
            Header {}
            Outlet::<Routes> {}
            Footer {}
        }
    }
}

