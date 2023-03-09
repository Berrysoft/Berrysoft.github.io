#![feature(iter_intersperse)]

use chrono::{DateTime, FixedOffset, Utc};
use lol_alloc::{FreeListAllocator, LockedAllocator};
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew_router::prelude::*;

pub mod data;
mod layout;
mod page;

#[derive(Debug, Clone, PartialEq, Routable)]
enum AppRoute {
    #[at("/")]
    Index,
    #[at("/about")]
    About,
    #[at("/blog/:name")]
    BlogDetail { name: String },
    #[at("/blog")]
    Blog,
    #[not_found]
    #[at("/notfound")]
    NotFound,
}

struct AppRoot;

impl Component for AppRoot {
    type Message = ();

    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let render = move |switch: AppRoute| {
            let location = gloo_utils::window().location().to_string().as_string();
            let location = location.and_then(|s| url::Url::parse(&s).ok());
            match switch {
                AppRoute::Index => html! {<page::IndexPage />},
                AppRoute::BlogDetail { name } => {
                    html! {<page::BlogDetailPage name={name} />}
                }
                AppRoute::Blog => html! {<page::BlogPage />},
                AppRoute::NotFound => html! {<page::NotFoundPage route={location} />},
                AppRoute::About => html! {<page::AboutPage />},
            }
        };
        html! {
            <BrowserRouter>
                <Switch<AppRoute> render={render}/>
            </BrowserRouter>
        }
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::new(
        #[cfg(debug_assertions)]
        log::Level::Debug,
        #[cfg(not(debug_assertions))]
        log::Level::Info,
    ));
    let element = gloo_utils::document()
        .query_selector("app")
        .unwrap()
        .unwrap();
    yew::Renderer::<AppRoot>::with_root(element).render();
}

#[global_allocator]
static ALLOC: LockedAllocator<FreeListAllocator> = LockedAllocator::new(FreeListAllocator::new());
