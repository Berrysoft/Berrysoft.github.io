use crate::*;
use reqwasm::http::Request;
use serde::de::DeserializeOwned;
use std::marker::PhantomData;
use url::Url;

pub struct FetcherBase<T: FetcherTypes> {
    data: FetcherData<T::StoreType>,
}

pub trait FetcherTypes {
    type TransferType;
    type StoreType;

    fn format_to_transfer(data: String) -> FetcherMessage<Self::TransferType>;
    fn transfer_to_store(data: Self::TransferType) -> Self::StoreType;
}

pub type FetcherMessage<T> = std::result::Result<T, reqwasm::Error>;
pub type FetcherData<T> = Option<FetcherMessage<T>>;

impl<T: FetcherTypes> FetcherBase<T> {
    async fn fetch(uri: &str) -> FetcherMessage<T::TransferType> {
        let res = Request::get(uri).send().await?;
        T::format_to_transfer(res.text().await?)
    }

    pub fn new<C: Component>(
        uri: &str,
        ctx: &Context<C>,
        cvt: impl Fn(FetcherMessage<T::TransferType>) -> C::Message + 'static,
    ) -> Self {
        let uri = Url::parse(&gloo_utils::window().location().origin().unwrap())
            .unwrap()
            .join(uri)
            .unwrap()
            .to_string();
        ctx.link()
            .send_future(async move { cvt(Self::fetch(&uri).await) });
        Self { data: None }
    }

    pub fn update(&mut self, msg: FetcherMessage<T::TransferType>) {
        self.data = Some(msg.map(|data| T::transfer_to_store(data)));
    }

    pub fn get(&self) -> Option<&T::StoreType> {
        match &self.data {
            Some(data) => match data {
                Ok(data) => Some(data),
                Err(err) => {
                    log::error!("{}", err);
                    None
                }
            },
            _ => None,
        }
    }
}

pub type JsonFetcher<T, W> = FetcherBase<JsonFetcherTypes<T, W>>;

pub type JsonFetcherMessage<T, W> =
    FetcherMessage<<JsonFetcherTypes<T, W> as FetcherTypes>::TransferType>;

pub struct JsonFetcherTypes<T: DeserializeOwned + 'static, W: From<T>> {
    _p: PhantomData<T>,
    _pw: PhantomData<W>,
}

impl<T: DeserializeOwned + 'static, W: From<T>> FetcherTypes for JsonFetcherTypes<T, W> {
    type TransferType = Vec<T>;

    type StoreType = Rc<Vec<W>>;

    fn format_to_transfer(data: String) -> FetcherMessage<Self::TransferType> {
        Ok(serde_json::from_str(&data)?)
    }

    fn transfer_to_store(data: Self::TransferType) -> Self::StoreType {
        Rc::new(data.into_iter().map(W::from).collect())
    }
}

pub type TextFetcher = FetcherBase<TextFetcherTypes>;

pub type TextFetcherMessage = FetcherMessage<String>;

pub struct TextFetcherTypes;

impl FetcherTypes for TextFetcherTypes {
    type TransferType = String;

    type StoreType = String;

    fn format_to_transfer(data: String) -> FetcherMessage<Self::TransferType> {
        Ok(data)
    }

    fn transfer_to_store(data: Self::TransferType) -> Self::StoreType {
        data
    }
}
