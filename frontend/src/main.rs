use dioxus::prelude::*;
use dioxus_logger::tracing::Level;
use frontend::components::App;

fn main() {
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    wasm_logger::init(wasm_logger::Config::default());
    console_error_panic_hook::set_once();

    launch(App);
}

