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
                h2 { "Loading cats..." }
            }
        };
    }
    rsx! {
        div {
            h2 { "Pick a cat" }
            ul {
                for cat in cats.unwrap() {
                    li {
                        Link { to: Routes::CatDetail { id: Uuid::parse_str(cat.clone().identifier.as_str()).unwrap() }, "{cat.name}" }
                        {cat.breed.clone()}
                    }
                }
            }
        }
    }
}

