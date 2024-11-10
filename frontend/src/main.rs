#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_logger::tracing::{debug, Level};
use dioxus_router::prelude::*;
// use shared::Cat;

fn main() {
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    wasm_logger::init(wasm_logger::Config::default());
    console_error_panic_hook::set_once();

    launch(App);
}

struct AppState {
    count: i32,
    text: String,
    // cats: Vec<Cat>,
}

impl AppState {
    async fn new() -> Self {
        // let cats = match reqwest::Client::new()
        //     .get("http://localhost:3000/cats")
        //     .send()
        //     .await
        // {
        //     Ok(res) => {
        //         println!("{:?}", res);
        //     },
        //     Err(e) => {
        //         eprintln!("{:?}", e);
        //     }
        // };

        Self {
            count: 0,
            text: String::from("..."),
            // cats
        }
    }
}

#[component]
fn App() -> Element {
    debug!("starting app");
    use_context_provider(|| Signal::new(AppState::new()));


    rsx! {
        Router::<Routes> {}
    }
}

#[component]
fn PageBase() -> Element {
    rsx! {
        div {
            Header {}
            Outlet::<Routes> {}
            Footer {}
        }
    }
}

#[component]
fn Header() -> Element {
    rsx! {
        header {
            h1 { "Dioxus" }
        }
    }
}

#[component]
fn Footer() -> Element {
    rsx! {
        footer {
            p { "© 2024 Dioxus" }
        }
    }
}

#[component]
fn Home() -> Element {
    let state = use_context::<Signal<AppState>>();

    rsx! {
        div {
            h1 { "High-Five counter: {state.read().count}" }
        }
    }
}

#[component]
fn About() -> Element {
    rsx! {
        div {
            h1 { "About" }
            p { "Dioxus is a full-stack web framework for Rust and WebAssembly." }
        }
    }
}

#[component]
fn PageNotFound(route: Vec<String>) -> Element {
    rsx! {
        div {
            h1 { "404" }
            p { "Page not found: {route:?}" }
        }
    }
}

#[component]
fn NavBar() -> Element {
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

#[component]
fn Cats() -> Element {
    rsx! {
        div {
            h1 { "Cats" }
            Outlet::<Routes> {}
        }
    }
}

#[component]
fn CatDetail(id: i32) -> Element {
    rsx! {
        div {
            h2 { "Cat {id}" }
        }
    }
}

#[component]
fn CatList() -> Element {
    rsx! {
        div {
            h2 { "Pick a cat" }
            ul {
                li {
                    Link { to: Routes::CatDetail { id: 1 }, "Cat 1" }
                }
                li {
                    Link { to: Routes::CatDetail { id: 2 }, "Cat 2" }
                }
            }
        }
    }
}

#[rustfmt::skip]
#[derive(Routable, PartialEq, Clone, Debug)]
enum Routes {
    #[layout(NavBar)]
        #[route("/")]
        Home {},
        #[route("/about")]
        About {},
        #[nest("/cats")]
            #[layout(Cats)]
                #[route("/")]
                CatList {},
                #[route("/:id")]
                CatDetail { id: i32 },
            #[end_layout]
        #[end_nest]
    #[end_layout]
    #[route("/:..route")]
    PageNotFound { route: Vec<String> },
}
