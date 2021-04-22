use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

mod index;

#[wasm_bindgen(start)]
pub fn run_app() {
    set_panic_hook();
    App::<index::IndexPage>::new().mount_to_body();
}

pub fn set_panic_hook() {
    console_error_panic_hook::set_once();
}

#[macro_export]
macro_rules! console_log {
    ($($t:tt)*) => {
        #[allow(unused_unsafe)]
        unsafe { web_sys::console::log_1(&format_args!($($t)*).to_string().into()) }
    }
}

pub fn base_url() -> Option<String> {
    web_sys::window().and_then(|w| w.location().origin().ok())
}
