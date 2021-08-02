use crate::command::Command;
use async_trait::async_trait;
use serde::de::DeserializeOwned;
use wasm_bindgen::{prelude::*, JsCast};
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};

pub struct Fetch<Data, Message> {
    url: String,
    handler: Box<dyn Fn(Result<Data, JsValue>) -> Message>,
}

impl<Data, Message> Fetch<Data, Message>
where
    Data: DeserializeOwned,
{
    pub fn new<Handler>(url: &str, handler: Handler) -> Self
    where
        Handler: 'static + Fn(Result<Data, JsValue>) -> Message,
    {
        Self {
            url: url.into(),
            handler: Box::new(handler),
        }
    }

    async fn perform(&self) -> Result<Data, JsValue> {
        let mut opts = RequestInit::new();
        opts.method("GET");
        opts.mode(RequestMode::Cors);

        let request = Request::new_with_str_and_init(&self.url, &opts).unwrap();

        let window = web_sys::window().unwrap();

        let response: Response = JsFuture::from(window.fetch_with_request(&request))
            .await?
            .dyn_into()?;
        let json = JsFuture::from(response.json()?).await?;

        json.into_serde().map_err(|e| e.to_string().into())
    }
}

#[async_trait(?Send)]
impl<Data, Message> Command<Message> for Fetch<Data, Message>
where
    Data: DeserializeOwned,
{
    async fn run(&self) -> Message {
        let result = self.perform().await;
        (self.handler)(result)
    }
}
