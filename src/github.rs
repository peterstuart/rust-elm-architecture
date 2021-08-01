use crate::{
    command::{self, Command},
    fetch::Fetch,
};
use serde::{Deserialize, Serialize};
use wasm_bindgen::JsValue;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct PullRequest {
    pub number: u32,
    pub state: String,
    pub title: String,
}

pub fn get_pull_requests<Message, Handler>(repo: &str, handler: Handler) -> command::Boxed<Message>
where
    Message: 'static,
    Handler: 'static + Fn(Result<Vec<PullRequest>, JsValue>) -> Message,
{
    let url = format!("https://api.github.com/repos/{}/pulls?state=all", repo);
    Fetch::new(&url, move |result| handler(result)).boxed()
}
