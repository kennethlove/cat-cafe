use dioxus::prelude::*;
use crate::components::{Button, Navbar};

#[component]
pub fn Header() -> Element {
    rsx! {
        header {
            class: "bg-white shadow fixed w-full",
            div {
                class: "mx-auto flex h-16 max-w-screen-xl items-center gap-8 px-4 sm:px-6 lg:px-8",
                a {
                    class: "block text-teal-600",
                    span {
                        class: "sr-only",
                        "Home"
                    }
                    img {
                        class: "h-10",
                        src: "/logo.png",
                    }
                }

                div {
                    class: "flex flex-1 items-center justify-end md:justify-between",
                    Navbar {}

                    div {
                        class: "flex items-center gap-4",
                        div {
                            class: "sm:flex sm:gap-4",
                            Button {
                                text: "Login".to_string(),
                                r#type: "button".to_string(),
                            }
                            Button {
                                text: "Sign up".to_string(),
                                r#type: "button".to_string(),
                            }
                        }
                    }
                }
            }
        }
    }
}

