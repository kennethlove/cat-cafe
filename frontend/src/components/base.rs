use dioxus::prelude::*;
use crate::routes::Routes;
use crate::components::{Header, Footer};

#[component]
pub fn Base() -> Element {
    rsx! {
        Header {}
        Outlet::<Routes> {}
        Footer {}
    }
}

