use dioxus::prelude::*;

use crate::components::{InputProps, Input};

#[component]
pub fn InputWithLabel(props: InputProps) -> Element {
    rsx! {
        label {
            r#for: "{props.name}",
            class: "relative block overflow-hidden border-b border-gray-200 bg-transparent pt-3 focus:within:border-blue-600",

            {Input(props.clone())}

            span {
                class: "absolute start-0 top-2 -translate-y-1/2 text-xs text-gray-700 transtion-all peer-placeholder-shown:top-1/2 peer-placeholder-show:text-sm peer-focus:top-2 peer-focus:text-xs",
                "{props.name.to_uppercase()}"
            }
        }
    }
}