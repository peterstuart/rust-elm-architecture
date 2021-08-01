mod app;
mod command;
mod fetch;
mod github;
mod renderer;
mod virtual_dom;

use app::App;
use command::Commands;
use github::PullRequest;
use virtual_dom::{Attribute, Html};
use wasm_bindgen::prelude::*;

#[derive(Clone, Debug, Eq, PartialEq)]
enum DataLoading<Data> {
    Loading,
    Loaded(Data),
    Error,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Model {
    counter: i32,
    text: String,
    pull_requests: DataLoading<Vec<PullRequest>>,
}

impl Model {
    fn increment(&self) -> Self {
        Self {
            counter: self.counter + 1,
            text: self.text.clone(),
            pull_requests: self.pull_requests.clone(),
        }
    }

    fn decrement(&self) -> Self {
        Self {
            counter: self.counter - 1,
            text: self.text.clone(),
            pull_requests: self.pull_requests.clone(),
        }
    }

    fn change_text(&self, text: &str) -> Self {
        Self {
            counter: self.counter,
            text: text.into(),
            pull_requests: self.pull_requests.clone(),
        }
    }

    fn change_pull_requests_data_loading(
        &self,
        data_loading: DataLoading<Vec<PullRequest>>,
    ) -> Self {
        Self {
            counter: self.counter,
            text: self.text.clone(),
            pull_requests: data_loading,
        }
    }
}

impl Default for Model {
    fn default() -> Self {
        Self {
            counter: 0,
            text: "".into(),
            pull_requests: DataLoading::Loading,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum Message {
    Increment,
    Decrement,
    ChangeText(String),
    ChangePullRequestsDataLoading(DataLoading<Vec<PullRequest>>),
}

fn init() -> (Model, Commands<Message>) {
    (
        Model::default(),
        vec![github::get_pull_requests(
            "peterstuart/rust-elm-architecture",
            |result| {
                Message::ChangePullRequestsDataLoading(
                    result.map_or_else(|_| DataLoading::Error, DataLoading::Loaded),
                )
            },
        )],
    )
}

fn update(message: &Message, model: &Model) -> (Model, Commands<Message>) {
    let model = match message {
        Message::Increment => model.increment(),
        Message::Decrement => model.decrement(),
        Message::ChangeText(text) => model.change_text(text),
        Message::ChangePullRequestsDataLoading(data_loading) => {
            model.change_pull_requests_data_loading(data_loading.clone())
        }
    };

    (model, vec![])
}

fn view(model: &Model) -> Html<Message> {
    Html::div(
        vec![],
        vec![
            Html::div(
                vec![],
                vec![
                    Html::button(
                        vec![Attribute::on_click(Message::Decrement)],
                        vec![Html::text("-")],
                    ),
                    Html::span(
                        vec![Attribute::class("counter")],
                        vec![Html::text(&model.counter.to_string())],
                    ),
                    Html::button(
                        vec![Attribute::on_click(Message::Increment)],
                        vec![Html::text("+")],
                    ),
                ],
            ),
            Html::input(
                vec![
                    Attribute::type_("text"),
                    Attribute::value(&model.text),
                    Attribute::on_input(|text| Message::ChangeText(text.into())),
                ],
                vec![],
            ),
            Html::text(&model.text),
            pull_requests_view(&model.pull_requests),
        ],
    )
}

fn pull_requests_view(data_loading: &DataLoading<Vec<PullRequest>>) -> Html<Message> {
    Html::div(
        vec![],
        match data_loading {
            DataLoading::Loading => vec![Html::text("Loading...")],
            DataLoading::Error => vec![Html::text("Error")],
            DataLoading::Loaded(pull_requests) => {
                pull_requests.iter().map(pull_request_view).collect()
            }
        },
    )
}

fn pull_request_view(pull_request: &PullRequest) -> Html<Message> {
    Html::p(
        vec![],
        vec![Html::text(&format!(
            "#{} {}: {}",
            pull_request.number, pull_request.title, pull_request.state
        ))],
    )
}

#[wasm_bindgen]
pub fn run_app() {
    init_log();
    set_panic_hook();

    let app = App::new(init, update, view, "root");
    app.start();
}

fn init_log() {
    #[cfg(feature = "console_log")]
    use log::Level;
    console_log::init_with_level(Level::Trace).expect("error initializing log");
}

fn set_panic_hook() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}
