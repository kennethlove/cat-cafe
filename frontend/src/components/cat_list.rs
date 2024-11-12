use dioxus::prelude::*;
use shared::Cat;
use crate::routes::Routes;
use uuid::Uuid;

#[component]
pub fn CatList() -> Element {
    let cats_resource = use_resource(|| async move {
        let response = reqwest::get("http://localhost:3000/cats").await.unwrap();
        let json = response.json::<Vec<Cat>>().await.unwrap();
        json
    });
    let cats = cats_resource.value().read().clone();

    if cats.is_none() {
        return rsx! {
            div {
                h2 {
                    class: "text-2xl font-bold text-yellow-800 dark:text-yellow-800 tracking-wider",
                    "Loading cats..."
                }
            }
        };
    }
    rsx! {
        div {
            h2 {
                class: "text-2xl font-bold text-yellow-800 dark:text-yellow-800 tracking-wider",
                "Pick a cat"
            }
            ul {
                for cat in cats.unwrap() {
                    li {
                        class: "text-lg font-bold text-yellow-800 dark:text-yellow-800 tracking-wider",
                        Link { to: Routes::CatDetail { id: Uuid::parse_str(cat.clone().identifier.as_str()).unwrap() }, "{cat.name}" }
                        span {
                            class: "text-sm text-yellow-800 dark:text-yellow-800 tracking-wider",
                            " - {cat.breed}"
                        }
                    }
                }
            }
        }
    }
}

