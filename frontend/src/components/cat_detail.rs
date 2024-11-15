use dioxus::prelude::*;
use uuid::Uuid;
use shared::Cat;

#[component]
pub fn CatDetail(id: Uuid) -> Element {
    let cat = use_resource(move || async move {
        let response = reqwest::get(format!("http://localhost:3000/cats/{}", id))
            .await.unwrap();
        let json = response.json::<Cat>().await.unwrap();
        json
    }).value().read().clone().unwrap_or_default();

    rsx! {
        div {
            h2 { "{cat.name}" }
            if let Some(image) = &cat.image {
                img {
                    src: image.to_string(),
                    alt: "{cat.name}",
                    class: "w-1/2"
                }
            }
        }
    }
}

