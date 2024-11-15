use std::rc::Rc;
use std::sync::Arc;
use dioxus::html::FileEngine;
use dioxus::prelude::*;
use dioxus_logger::tracing;
use reqwest::multipart::Part;
use uuid::Uuid;
use shared::{Cat, FILE_UPLOAD_PATH};
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
        form {
            class: "space-y-4 grid grid-cols-2 gap-4",
            enctype: "multipart/form-data",
            onsubmit: move |_| {
                let name = name_signal.read().clone();
                let breed: String = breed_signal.read().clone();
                let microchip: String = microchip_signal.read().clone();
                let mut image_path = None;
                let identifier = Uuid::new_v4().to_string();
                let url = format!("http://localhost:3000/cats/{}/images", identifier.clone());

                if !files_uploaded.read().is_empty() {
                    let files = files_uploaded.read();
                    let file = files.first().unwrap();
                    let name = file.name.clone();
                    let contents = file.contents.clone();

                    image_path = Some(format!("/files/{}", {
                        let extension = name.split('.').last().unwrap();
                        format!("{}.{}", identifier.clone(), extension)
                    }));

                    let _ = use_resource(move || {
                        let url = url.clone();
                        let name = name.clone();
                        let contents = contents.clone();

                        async move {
                            let upload = Part::bytes(contents.clone())
                                .file_name(name);
                            let form = reqwest::multipart::Form::new()
                                .part("fileupload", upload);
                            let client = reqwest::Client::new();
                            let response = client.post(url.as_str())
                                .multipart(form)
                                .send()
                                .await.unwrap();
                            if response.status().is_success() {
                                let _ = response.json::<String>().await.unwrap();
                            }
                        }
                    });
                }

                let new_cat = Cat {
                    identifier: identifier.clone(),
                    name,
                    breed,
                    microchip: Some(microchip),
                    image: image_path.into(),
                };

                let _ = use_resource(move || {
                    let value = new_cat.clone();
                    async move {
                        let client = reqwest::Client::new();
                        let response = client.post("http://localhost:3000/cats")
                            .json(&value)
                            .send()
                            .await.unwrap();
                        if response.status().is_success() {
                            let cat = response.json::<Cat>().await.unwrap();
                            name_signal.set(String::new());
                            breed_signal.set(String::new());
                            microchip_signal.set(String::new());
                            let nav = navigator();
                            nav.push(Routes::CatDetail { id: Uuid::parse_str(&cat.identifier.as_str()).unwrap() });
                        } else {
                            tracing::info!("Failed to create cat");
                        }
                    }
                });
            },
            h2 {
                class: "col-span-2 text-2xl",
                "Create a new cat"
            }
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
            div {
                class: "col-span-2",
                InputWithLabel {
                    r#type: "text".to_string(),
                    name: "microchip".to_string(),
                    value: microchip_signal.read().clone(),
                    placeholder: "Microchip".to_string(),
                    required: false,
                    oninput: move |event: Rc<FormData>| microchip_signal.set(event.value().clone()),
                    extra_css_classes: None,
                }
            }
            div {
                class: "col-span-2",
                "Upload an image",
                input {
                    class: "peer h-8 w-full border-none bg-transparent p-0 placeholder-transparent focus:border-transparent focus:outline-none focus:ring-0 sm:text-sm",
                    r#type: "file",
                    id: "images",
                    name: "images",
                    onchange: upload_files,
                }
            }

            Button {
                text: "Create".to_string(),
                r#type: "submit".to_string(),
            }
        }
    }
}