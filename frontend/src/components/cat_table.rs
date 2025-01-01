use std::fmt::Display;
use std::str::FromStr;
use dioxus::prelude::*;
use reqwest::{Client, Method, Request, RequestBuilder, Url};
use shared::{Cat, CatStatus};
use crate::routes::Routes;
use uuid::Uuid;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Eq, Deserialize, Serialize)]
enum FilterByStatus {
    #[default]
    All,
    New,
    Waiting,
    InCafe,
    Fostered,
    Adopted,
}

impl Display for FilterByStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FilterByStatus::All => write!(f, "all"),
            FilterByStatus::New => write!(f, "new"),
            FilterByStatus::Waiting => write!(f, "waiting"),
            FilterByStatus::InCafe => write!(f, "in-cafe"),
            FilterByStatus::Fostered => write!(f, "fostered"),
            FilterByStatus::Adopted => write!(f, "adopted"),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
enum SortByField {
    Name,
    Breed,
    Microchip,
}

impl Display for SortByField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SortByField::Name => write!(f, "name"),
            SortByField::Breed => write!(f, "breed"),
            SortByField::Microchip => write!(f, "microchip"),
        }
    }
}

impl FromStr for SortByField {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "name" => Ok(SortByField::Name),
            "breed" => Ok(SortByField::Breed),
            "microchip" => Ok(SortByField::Microchip),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
enum SortByDirection {
    Ascending,
    Descending,
}

#[derive(Clone, Debug, PartialEq, Props)]
struct CatTableHeaderProps {
    text: String,
    field: SortByField,
    sort_by_signal: Signal<SortByField>,
    sort_by_direction: Signal<SortByDirection>
}

#[component]
pub fn CatTableHeader(props: CatTableHeaderProps) -> Element {
    let mut sort_by = props.sort_by_signal;
    let mut sort_direction = props.sort_by_direction;

    let mut classes: String;

    // if the field and the signal match
    if sort_by.read().clone() == props.field.clone() {
        classes = "text-primary".to_string();
    } else {
        classes = "".to_string();
    }


    rsx! {
        th {
            onclick: move |_| {
                if sort_by.read().clone() == props.field.clone() {
                    let direction = match sort_direction.read().clone() {
                        SortByDirection::Ascending => SortByDirection::Descending,
                        SortByDirection::Descending => SortByDirection::Ascending,
                    };
                    sort_direction.set(direction);
                } else {
                    sort_direction.set(SortByDirection::Ascending);
                }
                sort_by.set(props.field.clone());
            },
            class: "cursor-pointer",
            span {
                class: "pr-4 uppercase {classes}",
                "{props.text}"
            }
            span {
                class: "material-symbols-rounded align-bottom",
                if sort_by.read().clone() == props.field.clone() {
                    match sort_direction.read().clone() {
                        SortByDirection::Ascending => "arrow_upward",
                        SortByDirection::Descending => "arrow_downward",
                    }
                } else {
                    "arrow_upward"
                }
            }
        }
    }
}

#[component]
pub fn CatTable() -> Element {
    let sort_by = use_signal(|| SortByField::Name);
    let sort_direction = use_signal(|| SortByDirection::Ascending);
    let mut filter = use_signal(|| FilterByStatus::All);

    let cats_resource = use_resource(move || async move {
        let url = Url::parse("http://localhost:3000/cats");
        let client = Client::new();
        let request = Request::new(Method::GET, url.ok().unwrap());
        let req = RequestBuilder::from_parts(client, request);

        let mut req = req.query(&[("sort_by_field", sort_by.read().to_owned())]);

        match sort_direction.read().to_owned() {
            SortByDirection::Descending => { req = req.query(&[("sort_direction", "desc")]); }
            _ => { req = req.query(&[("sort_direction", "asc")]) }
        }

        let req = req.query(&[("filter_by_status", filter.read().to_owned())]);

        let response = req.send().await.unwrap();

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
                "Cats"
            }
            div {
                class: "flex flex-wrap flex-row gap-4 sm:flex-nowrap",
                table {
                    class: "table table-zebra table-sm w-3/4 grow",
                    thead {
                        tr {
                            th {
                                class: "w-16",
                                input {
                                    r#type: "checkbox",
                                    name: "cat",
                                    value: "all",
                                    class: "checkbox",
                                }
                            }
                            th {
                                class: "w-16",
                            }
                            CatTableHeader { text: "Name", field: SortByField::Name, sort_by_signal: sort_by, sort_by_direction: sort_direction }
                            CatTableHeader { text: "Breed", field: SortByField::Breed, sort_by_signal: sort_by, sort_by_direction: sort_direction }
                            CatTableHeader { text: "Microchip #", field: SortByField::Microchip, sort_by_signal: sort_by, sort_by_direction: sort_direction }
                        }
                    }
                    tbody {
                        for cat in cats.clone().unwrap() {
                            tr {
                                td {
                                    input {
                                        r#type: "checkbox",
                                        name: "cat",
                                        value: "{cat.identifier}",
                                        class: "checkbox",
                                    }
                                }
                                td {
                                    {
                                        let status = match cat.clone().status {
                                            CatStatus::New => { "primary" },
                                            CatStatus::Waiting => { "secondary" },
                                            CatStatus::InCafe => { "accent" },
                                            CatStatus::Fostered => { "info" },
                                            CatStatus::Adopted => { "success" },
                                        };
                                        rsx! {
                                            div {
                                                class: "avatar indicator",
                                                div {
                                                    class: "w-12 rounded-sm",
                                                    span {
                                                        class: "indicator-item status status-lg status-{status}",
                                                        title: "{cat.clone().status}",
                                                    }
                                                    img {
                                                        class: "",
                                                        src: cat.clone().image.unwrap_or("https://placecats.com/96/96".to_string()),
                                                        alt: "{cat.name}"
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                                td {
                                    Link {
                                        to: Routes::CatDetail {
                                            id: Uuid::parse_str(cat.clone().identifier.as_str()).unwrap()
                                        },
                                        class: "link",
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

                aside {
                    class: "shrink min-w-fit max-w-1/4 sm:w-full",
                    section {
                        class: "p-2 bg-base-200 border-base-300 text-base-content text-sm rounded-sm",
                        h1 {
                            class: "font-bold",
                            "Filter by status"
                        }
                        ul {
                            li {
                                class: {format!("link {}", if filter.read().clone() == FilterByStatus::All { "text-primary" } else { "" })},
                                onclick: move |_| {
                                    filter.set(FilterByStatus::All);
                                },
                                "All"
                            }
                            li {
                                class: {format!("link {}", if filter.read().clone() == FilterByStatus::New { "text-primary" } else { "" })},
                                onclick: move |_| {
                                    filter.set(FilterByStatus::New);
                                },
                                "New"
                            }
                            li {
                                class: {format!("link {}", if filter.read().clone() == FilterByStatus::Waiting { "text-primary" } else { "" })},
                                onclick: move |_| {
                                    filter.set(FilterByStatus::Waiting);
                                },
                                "Waiting"
                            }
                            li {
                                class: {format!("link {}", if filter.read().clone() == FilterByStatus::InCafe { "text-primary" } else { "" })},
                                onclick: move |_| {
                                    filter.set(FilterByStatus::InCafe);
                                },
                                "In a Cafe"
                            }
                            li {
                                class: {format!("link {}", if filter.read().clone() == FilterByStatus::Fostered { "text-primary" } else { "" })},
                                onclick: move |_| {
                                    filter.set(FilterByStatus::Fostered);
                                },
                                "Fostered"
                            }
                            li {
                                class: {format!("link {}", if filter.read().clone() == FilterByStatus::Adopted { "text-primary" } else { "" })},
                                onclick: move |_| {
                                    filter.set(FilterByStatus::Adopted);
                                },
                                "Adopted"
                            }
                        }
                    }
                }
            }
        }
    }
}

