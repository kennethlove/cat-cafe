#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_logger::tracing::Level;
use uuid::Uuid;
use shared::Cat;

fn main() {
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    wasm_logger::init(wasm_logger::Config::default());
    console_error_panic_hook::set_once();

    launch(App);
}

#[derive(Clone, Debug)]
struct AppState {
    count: i32,
    text: String,
    cats: Signal<Vec<Cat>>,
}

impl AppState {
    fn new() -> Self {
        let cats = use_signal(|| async move {
            reqwest::get("http://localhost:3000/cats")
                .await
                .unwrap()
                .json::<Vec<Cat>>()
                .await
        });

        dioxus_logger::tracing::info!("cats: {:?}", cats);

        Self {
            count: 0,
            text: String::from("loading cats"),
            cats,
        }
    }
}

#[component]
fn App() -> Element {
    let state = AppState::new();
    use_context_provider(|| state);

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
    let state = use_context::<AppState>();
    rsx! {
        div {
            h1 { "Home" }
            p { "Welcome to Dioxus!" }
            p { "Count: {state.count}" }
            p { "Text: {state.text}" }
            p { "Cats: {state.cats:?}" }
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
fn CatDetail(id: Uuid) -> Element {
    rsx! {
        div {
            h2 { "Cat {id}" }
        }
    }
}

#[component]
fn CatList() -> Element {
    let state = use_context::<AppState>();
    dioxus_logger::tracing::info!("cats: {:?}", state.cats);
    rsx! {
        div {
            h2 { "Pick a cat" }
            ul {
                for cat in state.cats.iter() {
                    li {
                        Link { to: Routes::CatDetail { id: Uuid::parse_str(cat.clone().identifier.as_str()).unwrap() }, "{cat.name}" }
                        {cat.breed.clone()}
                    }
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
                CatDetail { id: Uuid },
            #[end_layout]
        #[end_nest]
    #[end_layout]
    #[route("/:..route")]
    PageNotFound { route: Vec<String> },
}
