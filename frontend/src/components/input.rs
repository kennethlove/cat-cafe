use std::rc::Rc;
use dioxus::prelude::*;

#[derive(Debug, PartialEq, Clone, Props)]
pub struct InputProps {
    pub extra_css_classes: Option<String>,
    pub name: String,
    pub placeholder: String,
    pub required: bool,
    pub r#type: String,
    pub value: String,
    pub oninput: EventHandler<Rc<FormData>>,
}

#[component]
pub fn Input(props: InputProps) -> Element {
    rsx! {
        input {
            class: "peer h-8 w-full border-none bg-transparent p-0 placeholder-transparent focus:border-transparent focus:outline-none focus:ring-0 sm:text-sm {props.extra_css_classes.unwrap_or_default()}",
            r#type: "{props.r#type}",
            id: "{props.name}",
            name: "{props.name}",
            placeholder: "{props.placeholder}",
            value: "{props.value}",
            oninput: move |event| {
                props.oninput.call(event.data())
            },
            if props.required {
                "required: true,"
            }
        }
    }
}