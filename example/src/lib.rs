
use wasm_bindgen::prelude::*;
mod app;

#[wasm_bindgen]
pub fn run() {
    lollipop::run::<app::Msg>(app::app())
}
