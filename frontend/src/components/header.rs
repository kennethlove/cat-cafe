use dioxus::prelude::*;
use crate::components::Navbar;
use crate::routes::Routes;

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
                            Link {
                                class: "group inline-block rounded bg-gradient-to-r from-pink-500 via-red-500 to-yellow-500 p-[2px] hover:text-white focus:outline-none focus:ring active:text-opacity-75",
                                to: Routes::CatCreateForm {},
                                span {
                                    class: "block rounded-sm bg-white px-8 py-3 text-sm font-medium group-hover:bg-transparent",

                                    "Add a cat"
                                }
                            }
                            Link {
                                class: "group inline-block rounded bg-gradient-to-r from-pink-500 via-red-500 to-yellow-500 p-[2px] hover:text-white focus:outline-none focus:ring active:text-opacity-75",
                                to: "#",
                                span {
                                    class: "block rounded-sm bg-white px-8 py-3 text-sm font-medium group-hover:bg-transparent",

                                    "Add a cafe"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

