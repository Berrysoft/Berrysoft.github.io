use crate::*;
use http::Uri;
use serde::de::DeserializeOwned;
use std::convert::TryFrom;
use yew::format::{Json, Nothing};
use yew::services::fetch::{FetchService, FetchTask, Request, Response};

pub struct Fetcher<T: DeserializeOwned + 'static> {
    data: FetcherData<T>,
    #[allow(dead_code)]
    task: FetchTask,
}

#[derive(Debug)]
enum FetcherData<T: DeserializeOwned + 'static> {
    None,
    Some(Arc<Vec<T>>),
    Err(anyhow::Error),
}

pub type FetcherMessage<T> = anyhow::Result<Vec<T>>;

impl<T: DeserializeOwned + 'static> Fetcher<T> {
    pub fn new<C: Component, U>(
        uri: U,
        link: ComponentLink<C>,
        cvt: impl Fn(FetcherMessage<T>) -> C::Message + 'static,
    ) -> Self
    where
        Uri: TryFrom<U>,
        http::Error: From<<Uri as TryFrom<U>>::Error>,
    {
        let handler = link.callback(move |res: Response<Json<FetcherMessage<T>>>| {
            let (_, Json(data)) = res.into_parts();
            cvt(data)
        });
        let req = Request::get(uri).body(Nothing).unwrap();
        let task = FetchService::fetch(req, handler).unwrap();
        Self {
            data: FetcherData::None,
            task,
        }
    }

    pub fn update(&mut self, msg: FetcherMessage<T>) {
        match msg {
            Ok(data) => {
                self.data = FetcherData::Some(Arc::new(data));
            }
            Err(err) => self.data = FetcherData::Err(err),
        }
    }

    pub fn get(&self) -> Option<Arc<Vec<T>>> {
        match &self.data {
            FetcherData::Some(data) => Some(data.clone()),
            _ => None,
        }
    }
}
