use dioxus::prelude::*;
use crate::components::Navbar;

#[component]
pub fn Header() -> Element {
    rsx! {
        header {
            class: "bg-white",
            div {
                class: "mx-auto flex h-16 max-w-screen-xl items-center gap-8 px-4 sm:px-6 lg:px-8",
                a {
                    class: "block text-teal-600",
                    span {
                        class: "sr-only",
                        "Home"
                    }
                    img {
                        class: "h-16",
                        src: "https://placecats.com/64/64",
                    }
                }

                div {
                    class: "flex flex-1 items-center justify-end md:justify-between",
                    Navbar {}

                    div {
                        class: "flex items-center gap-4",
                        div {
                            class: "sm:flex sm:gap-4",
                            a {
                                class: "block rounded-md bg-rose-600 px-6 py-2.5 text-sm font-medium text-white transition hover:bg-rose-700 cursor-pointer",
                                "Login"
                            }
                            a {
                                class: "hidden rounded-md bg-gray-100 px-5 py-2.5 text-sm font-medium text-rose-600 transition hover:text-rose-600/75 sm:block cursor-pointer",
                                "Sign up"
                            }
                        }
                    }
                }
            }
        }
    }
}

