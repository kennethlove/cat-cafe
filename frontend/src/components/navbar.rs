use dioxus::prelude::*;
use crate::routes::Routes;

#[component]
pub fn Navbar() -> Element {
    rsx! {
        nav {
            aria_label: "Global",
            class: "hidden md:block",
            ul {
                class: "flex items-center gap-6 text-sm",
                li {
                    Link {
                        class: "text-content transition hover:text-rose-500",
                        to: Routes::Home {}, "Home"
                    }
                }
                li {
                    Link {
                        class: "text-content transition hover:text-rose-500",
                        to: Routes::CatTable {}, "Cats"
                    }
                }
                li {
                    Link {
                        class: "text-content transition hover:text-rose-500",
                        to: Routes::About {}, "About"
                    }
                }
            }
        }
    }
}
