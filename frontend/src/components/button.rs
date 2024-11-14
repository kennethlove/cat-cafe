use dioxus::prelude::*;

#[derive(Clone, Debug, PartialEq, Props)]
pub struct ButtonProps {
    text: String,
    r#type: String,
}

#[component]
pub fn Button(props: ButtonProps) -> Element {
    rsx! {
        button {
            class: "group inline-block rounded bg-gradient-to-r from-pink-500 via-red-500 to-yellow-500 p-[2px] hover:text-white focus:outline-none focus:ring active:text-opacity-75",
            r#type: "{props.r#type}",
            span {
                class: "block rounded-sm bg-white px-8 py-3 text-sm font-medium group-hover:bg-transparent",
                "{props.text}"
            }
        }
    }
}