use dioxus::prelude::*;
use crate::routes::Routes;
#[component]
pub fn Cats() -> Element {
    rsx! {
        div {
            h1 {
                class: "text-2xl font-bold",
                "Cats"
            }
            Outlet::<Routes> {}
        }
    }
}

