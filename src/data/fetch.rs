use crate::*;
use serde::de::DeserializeOwned;
use std::marker::PhantomData;
use yew::format::{Json, Nothing, Text};
use yew::services::fetch::{FetchService, FetchTask, Request, Response};

pub struct FetcherBase<T: FetcherTypes> {
    data: FetcherData<T::StoreType>,
    #[allow(dead_code)]
    task: FetchTask,
}

pub trait FetcherTypes {
    type FormatType: From<Text> + 'static;
    type TransferType;
    type StoreType;

    fn format_to_transfer(data: Self::FormatType) -> FetcherMessage<Self::TransferType>;
    fn transfer_to_store(data: Self::TransferType) -> Self::StoreType;
}

#[derive(Debug)]
enum FetcherData<T> {
    None,
    Some(T),
    Err(anyhow::Error),
}

pub type FetcherMessage<T> = anyhow::Result<T>;

impl<T: FetcherTypes> FetcherBase<T> {
    pub fn new<C: Component>(
        uri: &str,
        link: ComponentLink<C>,
        cvt: impl Fn(FetcherMessage<T::TransferType>) -> C::Message + 'static,
    ) -> Self {
        let handler = link.callback(move |res: Response<T::FormatType>| {
            let (_, data) = res.into_parts();
            let data = T::format_to_transfer(data);
            cvt(data)
        });
        let req = Request::get(uri).body(Nothing).unwrap();
        let task = FetchService::fetch(req, handler).unwrap();
        Self {
            data: FetcherData::None,
            task,
        }
    }

    pub fn update(&mut self, msg: FetcherMessage<T::TransferType>) {
        match msg {
            Ok(data) => self.data = FetcherData::Some(T::transfer_to_store(data)),
            Err(err) => self.data = FetcherData::Err(err),
        }
    }

    pub fn get(&self) -> Option<&T::StoreType> {
        match &self.data {
            FetcherData::Some(data) => Some(data),
            FetcherData::Err(err) => {
                log::error!("{}", err);
                None
            }
            _ => None,
        }
    }
}

pub type JsonFetcher<T> = FetcherBase<JsonFetcherTypes<T>>;

pub type JsonFetcherMessage<T> =
    FetcherMessage<<JsonFetcherTypes<T> as FetcherTypes>::TransferType>;

pub struct JsonFetcherTypes<T: DeserializeOwned + 'static> {
    _p: PhantomData<T>,
}

impl<T: DeserializeOwned + 'static> FetcherTypes for JsonFetcherTypes<T> {
    type FormatType = Json<anyhow::Result<Vec<T>>>;

    type TransferType = Vec<T>;

    type StoreType = Rc<Vec<T>>;

    fn format_to_transfer(data: Self::FormatType) -> FetcherMessage<Self::TransferType> {
        let Json(data) = data;
        data
    }

    fn transfer_to_store(data: Self::TransferType) -> Self::StoreType {
        Rc::new(data)
    }
}

pub type TextFetcher = FetcherBase<TextFetcherTypes>;

pub type TextFetcherMessage = FetcherMessage<String>;

pub struct TextFetcherTypes;

impl FetcherTypes for TextFetcherTypes {
    type FormatType = Text;

    type TransferType = String;

    type StoreType = String;

    fn format_to_transfer(data: Self::FormatType) -> FetcherMessage<Self::TransferType> {
        data
    }

    fn transfer_to_store(data: Self::TransferType) -> Self::StoreType {
        data
    }
}
