use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    rsx! {
      section {
        class: "relative bg-[url(https://images.unsplash.com/photo-1661518601049-469cb1293ae5?q=80&w=1174&auto=format&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D)] bg-cover bg-center bg-no-repeat",
        div {
          class: "absolute inset-0 bg-transparent sm:from-white/75 sm:to-white/25 sm:bg-gradient-to-r",
        }
        div {
          class: "relative mx-auto max-w-screen-xl px-4 py-32 sm:px-6 lg:flex lg:h-screen lg:items-center lg:px-8",
          div {
            class: "max-w-xl ltr:sm:text-left rtl:sm:text-right",
            h1 {
              class: "text-3xl font-extrabold text-rose-700 sm:text-5xl",
              "Let us find your"
              strong {
                class: "block font-extrabold text-rose-500",
                " Furrever Home."
              }
            }
            p {
              class: "mt-4 max-w-lg text-gray-700 sm:text-xl/relaxed",
              "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua."
            }

            div {
              class: "mt-8 flex flex-wrap gap-4 text-center",
              a {
                class: "block w-full rounded bg-rose-600 px-12 py-3 text-sm font-medium text-white shadow hover:bg-rose-700 focus:outline-none focus:ring active:bg-rose-500 sm:w-auto cursor-pointer",
                "Get started",
              }
              a {
                class: "block w-full rounded bg-white px-12 py-3 text-sm font-medium text-rose-600 shadow hover:text-rose-700 focus:outline-none focus:ring active:text-rose-100 sm:w-auto cursor-pointer",
                "Learn more",
              }
            }
          }
        }
      }
    }
}

