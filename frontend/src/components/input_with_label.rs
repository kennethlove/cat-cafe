use dioxus::prelude::*;

use crate::components::{InputProps, Input};

#[component]
pub fn InputWithLabel(props: InputProps) -> Element {
    rsx! {
        label {
            r#for: "{props.name}",
            class: "input",

            {Input(props.clone())}

            "{props.name.to_uppercase()}"
        }
    }
}
