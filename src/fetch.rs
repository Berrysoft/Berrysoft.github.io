use crate::*;
use http::Uri;
use serde::de::DeserializeOwned;
use std::convert::TryFrom;
use yew::format::{Json, Nothing, Text};
use yew::services::fetch::{FetchService, FetchTask, Request, Response};

pub struct JsonFetcher<T: DeserializeOwned + 'static> {
    data: FetcherData<Arc<Vec<T>>>,
    #[allow(dead_code)]
    task: FetchTask,
}

#[derive(Debug)]
enum FetcherData<T> {
    None,
    Some(T),
    Err(anyhow::Error),
}

pub type JsonFetcherMessage<T> = anyhow::Result<Vec<T>>;

impl<T: DeserializeOwned + 'static> JsonFetcher<T> {
    pub fn new<C: Component, U>(
        uri: U,
        link: ComponentLink<C>,
        cvt: impl Fn(JsonFetcherMessage<T>) -> C::Message + 'static,
    ) -> Self
    where
        Uri: TryFrom<U>,
        http::Error: From<<Uri as TryFrom<U>>::Error>,
    {
        let handler = link.callback(move |res: Response<Json<JsonFetcherMessage<T>>>| {
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

    pub fn update(&mut self, msg: JsonFetcherMessage<T>) {
        match msg {
            Ok(data) => self.data = FetcherData::Some(Arc::new(data)),
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

pub struct TextFetcher {
    data: FetcherData<String>,
    #[allow(dead_code)]
    task: FetchTask,
}

pub type TextFetcherMessage = Text;

impl TextFetcher {
    pub fn new<C: Component, U>(
        uri: U,
        link: ComponentLink<C>,
        cvt: impl Fn(TextFetcherMessage) -> C::Message + 'static,
    ) -> Self
    where
        Uri: TryFrom<U>,
        http::Error: From<<Uri as TryFrom<U>>::Error>,
    {
        let handler = link.callback(move |res: Response<Text>| {
            let (_, data) = res.into_parts();
            cvt(data)
        });
        let req = Request::get(uri).body(Nothing).unwrap();
        let task = FetchService::fetch(req, handler).unwrap();
        Self {
            data: FetcherData::None,
            task,
        }
    }

    pub fn update(&mut self, msg: TextFetcherMessage) {
        match msg {
            Ok(data) => self.data = FetcherData::Some(data),
            Err(err) => self.data = FetcherData::Err(err),
        }
    }

    pub fn get(&self) -> Option<String> {
        match &self.data {
            FetcherData::Some(data) => Some(data.clone()),
            _ => None,
        }
    }
}
