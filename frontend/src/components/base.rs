use dioxus::prelude::*;
use crate::routes::Routes;
use crate::components::{Header, Footer};

#[component]
pub fn Base() -> Element {
    rsx! {
        Header {}
        nav {
            ul {
                li {
                    Link { to: Routes::Home {}, "Home" }
                }
                li {
                    Link { to: Routes::CatList {}, "Cats" }
                }
                li {
                    Link { to: Routes::About {}, "About" }
                }
            }
        }
        Outlet::<Routes> {}
        Footer {}
    }
}

