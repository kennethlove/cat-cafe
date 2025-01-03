use std::rc::Rc;
use std::sync::Arc;
use dioxus::html::FileEngine;
use dioxus::prelude::*;
use dioxus_logger::tracing;
use reqwest::multipart::Part;
use reqwest::multipart::Form;
use uuid::Uuid;
use shared::{Cat, NewCat};
use crate::components::InputWithLabel;
use crate::components::Button;
use crate::routes::Routes;
use shared::UploadedFile;

#[component]
pub fn CatCreateForm() -> Element {
    let cat = use_signal(||Cat::default());
    let mut name_signal = use_signal(||cat.read().name.clone());
    let mut breed_signal = use_signal(||cat.read().breed.clone());
    let mut microchip_signal = use_signal(||cat.read().microchip.clone().unwrap_or_default());
    let mut files_uploaded = use_signal(|| Vec::new() as Vec<UploadedFile>);
    let mut image_path = use_signal(String::new);

    let read_files = move |file_engine: Arc<dyn FileEngine>| async move {
        let files = file_engine.files();
        for file_name in &files {
            if let Some(contents) = file_engine.read_file(file_name).await {
                files_uploaded.write().push(UploadedFile {
                    name: file_name.clone(),
                    contents: contents.clone(),
                });
            }
        }
    };

    let upload_files = move |event: FormEvent| async move {
        if let Some(file_engine) = event.files() {
            read_files(file_engine).await;
        }
    };

    rsx! {
        h2 {
            class: "col-span-2 text-2xl",
            "Create a new cat"
        }
        form {
            enctype: "multipart/form-data",
            onsubmit: move |_event| {
                let name = name_signal.read().clone();
                let breed: String = breed_signal.read().clone();
                let microchip: String = microchip_signal.read().clone();
                let identifier = Uuid::new_v4().to_string();
                let url = format!("http://localhost:3000/cats/{}/images", identifier.clone());

                if !files_uploaded.read().is_empty() {
                    let files = files_uploaded.read();
                    let file = files.first().unwrap();
                    let file_name = file.name.clone();
                    let contents = file.contents.clone();

                    let _ = use_resource(move || {
                        let url = url.clone();
                        let file_name = file_name.clone();
                        let contents = contents.clone();
                        let microchip = microchip.clone();
                        let breed = breed.clone();
                        let identifier = identifier.clone();
                        let cat_name = name.clone();

                        async move {
                            let upload = Part::bytes(contents)
                                .file_name(file_name.clone());
                            let form = Form::new()
                                .part("fileupload", upload);
                            let client = reqwest::Client::new();
                            let response = client.post(url)
                                .multipart(form)
                                .send()
                                .await.unwrap();
                            if response.status().is_success() {
                                response.headers().get("Location").map(|location| {
                                    image_path.set(location.to_str().unwrap().to_string());
                                });
                            }

                            let image_path = image_path.read().clone();

                            let new_cat = NewCat {
                                identifier: Some(identifier.clone()),
                                name: cat_name,
                                breed,
                                microchip: Some(microchip),
                                image: Some(image_path),
                            };

                            let value = new_cat.clone();

                            let client = reqwest::Client::new();
                            let response = client.post("http://localhost:3000/cats")
                                .json(&value)
                                .send()
                                .await.unwrap();
                            if response.status().is_success() {
                                let cat = response.json::<Cat>().await.unwrap();
                                let nav = navigator();
                                nav.push(Routes::CatDetail { id: Uuid::parse_str(&cat.identifier.as_str()).unwrap() });
                            } else {
                                tracing::info!("Failed to create cat");
                            }
                        }
                    });
                }
            },
            div {
                class: "flex flex-wrap gap-4",
                InputWithLabel {
                    r#type: "text".to_string(),
                    required: true,
                    value: name_signal.read().clone(),
                    name: "name".to_string(),
                    placeholder: "Name".to_string(),
                    oninput: move |event: Rc<FormData>| name_signal.set(event.value().clone()),
                    extra_css_classes: None,
                }
                InputWithLabel {
                    r#type: "text".to_string(),
                    required: true,
                    value: breed_signal.read().clone(),
                    name: "breed".to_string(),
                    placeholder: "Breed".to_string(),
                    oninput: move |event: Rc<FormData>| breed_signal.set(event.value().clone()),
                    extra_css_classes: None,
                }
                InputWithLabel {
                    r#type: "text".to_string(),
                    name: "microchip".to_string(),
                    value: microchip_signal.read().clone(),
                    placeholder: "Microchip".to_string(),
                    required: false,
                    oninput: move |event: Rc<FormData>| microchip_signal.set(event.value().clone()),
                    extra_css_classes: None,
                }
                label {
                    input {
                        class: "file-input",
                        r#type: "file",
                        id: "images",
                        name: "images",
                        onchange: upload_files,
                    }
                }

            }
            button {
                class: "btn btn-primary mt-4",
                r#type: "submit",
                "Create"
            }
        }
    }
}
