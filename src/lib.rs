use wasm_bindgen::prelude::*;
use yew::prelude::*;

mod index;

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<index::IndexPage>::new().mount_to_body();
}
