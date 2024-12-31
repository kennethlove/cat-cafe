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
            table {
                class: "table table-zebra table-auto",
                thead {
                    tr {
                        th {}
                        th { "image" }
                        th { "name" }
                        th { "breed" }
                        th { "microchip" }
                    }
                }
                tbody {
                    for cat in cats.clone().unwrap() {
                        tr {
                            td {
                                input {
                                    r#type: "checkbox",
                                    name: "cat",
                                    value: "{cat.identifier}"
                                }
                            }
                            td {
                                div {
                                    class: "avatar avatar-online",
                                    div {
                                        class: "w-16 rounded-full",
                                        img {
                                            class: "",
                                            src: cat.image.unwrap_or("https://placecats.com/96/96".to_string()),
                                            alt: "{cat.name}"
                                        }
                                    }
                                }
                            }
                            td {
                                Link {
                                    to: Routes::CatDetail {
                                        id: Uuid::parse_str(cat.clone().identifier.as_str()).unwrap()
                                    },
                                    "{cat.name}"
                                }
                            }
                            td {
                                "{cat.breed}"
                            }
                            td {
                                if !cat.microchip.is_some() { "None" }
                                    else { "{&cat.microchip.as_ref().unwrap()}" }
                            }
                        }
                    }
                }
            }
        }
    }
}

