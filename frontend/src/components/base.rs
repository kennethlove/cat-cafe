use dioxus::prelude::*;
use shared::Cat;
use crate::routes::Routes;
use crate::components::{Header, Footer, EditModal};

#[component]
pub fn Base() -> Element {
    let _: Signal<Option<Cat>> = use_context_provider(|| Signal::new(None));
    let edit_modal_signal: Signal<Option<Cat>> = use_context();

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

        EditModal {}

    }
}

