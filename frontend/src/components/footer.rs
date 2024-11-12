use dioxus::prelude::*;

#[component]
pub fn Footer() -> Element {
    rsx! {
        footer {
            class: "bg-gray-50",
            div {
                class: "mx-auto max-w-screen-xl px-4 py-8 sm:px-6 lg:px-8",
                div {
                    class: "sm:flex sm:items-center sm:justify-between",
                    div {
                        class: "flex justify-center text-teal-600 sm:justify-start",
                        img {
                            class: "h-16",
                            src: "https://placecats.com/64/64",
                        }
                    }
                    p {
                        class: "mt-4 text-center text-sm text-rose-600 lg:mt-0 lg:text-right",
                        "Copyright © 2024. All rights reserved."
                    }
                }
            }
        }
    }
}

