use crate::github::{self, PullRequest};
use rust_elm_architecture::{App, Attribute, Commands, Html};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DataLoading<Data> {
    Loading,
    Loaded(Data),
    Error,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Model {
    counter: i32,
    repo: String,
    pull_requests: DataLoading<Vec<PullRequest>>,
}

impl Default for Model {
    fn default() -> Self {
        Self {
            counter: 0,
            repo: "peterstuart/rust-elm-architecture".into(),
            pull_requests: DataLoading::Loading,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Message {
    Increment,
    Decrement,
    ChangeRepo(String),
    FetchPullRequests,
    ChangePullRequestsDataLoading(DataLoading<Vec<PullRequest>>),
}

fn init() -> (Model, Commands<Message>) {
    let model = Model::default();
    let repo = model.repo.clone();

    (
        model,
        vec![github::get_pull_requests(&repo, |result| {
            Message::ChangePullRequestsDataLoading(
                result.map_or_else(|_| DataLoading::Error, DataLoading::Loaded),
            )
        })],
    )
}

fn update(message: &Message, model: &mut Model) -> Commands<Message> {
    match message {
        Message::Increment => model.counter += 1,
        Message::Decrement => model.counter -= 1,
        Message::ChangeRepo(repo) => model.repo = repo.clone(),
        Message::FetchPullRequests => model.pull_requests = DataLoading::Loading,
        Message::ChangePullRequestsDataLoading(data_loading) => {
            model.pull_requests = data_loading.clone()
        }
    };

    match message {
        Message::FetchPullRequests => vec![github::get_pull_requests(&model.repo, |result| {
            Message::ChangePullRequestsDataLoading(
                result.map_or_else(|_| DataLoading::Error, DataLoading::Loaded),
            )
        })],
        _ => vec![],
    }
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
                    Attribute::value(&model.repo),
                    Attribute::on_input(|text| Message::ChangeRepo(text.into())),
                ],
                vec![],
            ),
            Html::button(
                vec![Attribute::on_click(Message::FetchPullRequests)],
                vec![Html::text("Fetch PRs")],
            ),
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

pub fn create(root_id: &str) -> App<Model, Message> {
    App::new(init, update, view, root_id)
}