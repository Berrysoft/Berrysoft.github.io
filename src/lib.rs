#![recursion_limit = "256"]

use chrono::{DateTime, FixedOffset, Utc};
use std::sync::Arc;
use wasm_bindgen::prelude::*;
use yew::format::{Json, Nothing};
use yew::prelude::*;
use yew::services::fetch::{FetchService, FetchTask, Request, Response};
use yew_router::prelude::*;

mod data;
mod datagrid;
mod fetch;
mod footer;
mod header;

mod about;
mod index;

#[derive(Debug, Clone, Copy, Switch)]
enum AppRoute {
    #[to = "/about"]
    About,
    #[to = "/"]
    Index,
}

struct AppRoot;

impl Component for AppRoot {
    type Message = ();

    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let render = Router::render(|switch: AppRoute| match switch {
            AppRoute::Index => html! {<index::IndexPage />},
            AppRoute::About => html! {<about::AboutPage />},
        });
        html! {
            <Router<AppRoute, ()> render=render/>
        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    set_panic_hook();
    App::<AppRoot>::new().mount_to_body();
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
