#![recursion_limit = "256"]

use chrono::{DateTime, FixedOffset, Utc};
use std::sync::Arc;
use wasm_bindgen::prelude::*;
use yew::format::{Json, Nothing};
use yew::prelude::*;
use yew::services::fetch::{FetchService, FetchTask, Request, Response};

mod data;
mod datagrid;
mod fetch;
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
