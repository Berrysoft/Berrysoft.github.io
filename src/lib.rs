#![recursion_limit = "512"]
#![feature(iter_intersperse)]

use chrono::{DateTime, FixedOffset, Utc};
use lazy_static::lazy_static;
use std::sync::Arc;
use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew_router::prelude::*;

pub mod data;
mod layout;
mod page;

#[derive(Debug, Clone, Switch)]
enum AppRoute {
    #[to = "/about"]
    About,
    #[to = "/blog/{*}"]
    BlogDetail(String),
    #[to = "/blog"]
    Blog,
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
            AppRoute::Index => html! {<page::IndexPage />},
            AppRoute::BlogDetail(name) => html! {<page::BlogDetailPage name=name />},
            AppRoute::Blog => html! {<page::BlogPage />},
            AppRoute::About => html! {<page::AboutPage />},
        });
        html! {
            <Router<AppRoute, ()> render=render/>
        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    yew::initialize();
    wasm_logger::init(wasm_logger::Config::new(
        #[cfg(debug_assertions)]
        log::Level::Debug,
        #[cfg(not(debug_assertions))]
        log::Level::Info,
    ));
    let element = yew::utils::document()
        .query_selector("app")
        .unwrap()
        .unwrap();
    App::<AppRoot>::new().mount(element);
    yew::run_loop();
}

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
