#![allow(non_snake_case)]
use oauth2_client::App;

fn main() {
    // launch the web app
    wasm_logger::init(wasm_logger::Config::default());
    dioxus_web::launch(App);
}
