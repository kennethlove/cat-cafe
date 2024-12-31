use dioxus::prelude::*;

#[component]
pub fn Footer() -> Element {
    rsx! {
        footer {
            class: "",
            div {
                class: "mx-auto max-w-screen-xl px-6 py-8 md:px-4 lg:px-8",
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
                        class: "mt-4 text-center text-sm text-rose-600 lg:mt-0 lg:text-right",
                        "Copyright © 2025. All rights reserved."
                    }
                }
            }
        }
    }
}

