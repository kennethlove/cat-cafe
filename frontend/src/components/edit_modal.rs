use dioxus::prelude::*;
use shared::Cat;
use crate::components::CatForm;

pub fn EditModal() -> Element {
    let mut edit_modal_signal: Signal<Option<Cat>> = use_context();

    rsx! {
        dialog {
            open: edit_modal_signal.read().clone().is_some(),
            class: "modal",
            div {
                class: "modal-box",
                h3 {
                    class: "text-lg font-bold",
                    "Edit cat"
                }

                if edit_modal_signal.read().clone().is_some() {
                    CatForm {}
                }

                div {
                    class: "modal-action",
                    form {
                        method: "dialog",
                        class: "flex flex-row items-end gap-4",
                        button {
                            onclick: move |_| { edit_modal_signal.set(None); },
                            class: "btn btn-primary",
                            "Save"
                        }
                        button {
                            onclick: move |_| { edit_modal_signal.set(None); },
                            class: "btn btn-secondary",
                            "Cancel"
                        }
                    }
                }
            }
        }
    }
}
