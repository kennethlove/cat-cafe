use dioxus::prelude::*;
use crate::components::Navbar;
use crate::routes::Routes;

#[component]
pub fn Header() -> Element {
    rsx! {
        header {
            class: "mt-4",
            div {
                class: "flex items-center gap-8",
                a {
                    class: "block",
                    span {
                        class: "sr-only",
                        "Cat Cafe"
                    }
                    img {
                        class: "h-10",
                        src: asset!("/assets/logo.png"),
                        alt: "Cat Cafe logo"
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
                                class: "btn btn-primary btn-outline",
                                to: Routes::CatCreateForm {},
                                "Add a cat"
                            }
                            Link {
                                class: "btn btn-primary btn-outline",
                                to: Routes::Home {},
                                "Add a cafe"
                            }
                        }
                    }
                }
            }
        }
    }
}

