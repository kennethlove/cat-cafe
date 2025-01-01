use dioxus::prelude::*;

#[component]
pub fn Footer() -> Element {
    rsx! {
        footer {
            class: "",
            div {
                class: "",
                div {
                    class: "flex items-center justify-between",
                    div {
                        class: "flex justify-center text-teal-600 sm:justify-start",
                        img {
                            class: "h-10",
                            src: asset!("/assets/cat.png"),
                        }
                    }
                    p {
                        class: "mt-4 text-center text-sm text-content lg:mt-0 lg:text-right",
                        "Copyright © 2025. All rights reserved."
                    }
                }
            }
        }
    }
}

