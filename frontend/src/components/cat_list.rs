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
                        class: "mb-6",
                        Link {
                            to: Routes::CatDetail {
                                id: Uuid::parse_str(cat.clone().identifier.as_str()).unwrap()
                            },
                            class: "block",
                            img {
                                class: "h-52 w-full rounded-bl-3xl rounded-tr-3xl object-cover sm:h-64 lg:h-72",
                                src: cat.image.unwrap_or("https://placecats.com/960/960".to_string()),
                                alt: "{cat.name}"
                            }
                            div {
                                class: "mt-2 sm:flex sm:items-center sm:justify-center sm:gap-4",
                                strong {
                                    class: "font-medium",
                                    "{cat.name}"
                                }
                                span {
                                    class: "hidden sm:block sm:h-px sm:w-8 sm:bg-pink-500",
                                    ""
                                }
                                p {
                                    class: "mt-0.5 opacity-50 sm:mt-0",
                                    "{cat.breed}"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

