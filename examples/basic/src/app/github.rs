use super::DataLoading;
use crate::github::{self, PullRequest};
use rust_elm_architecture::{Attribute, Commands, Html};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Model {
    repo: String,
    pull_requests: DataLoading<Vec<PullRequest>>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Message {
    ChangeRepo(String),
    FetchPullRequests,
    ChangePullRequestsDataLoading(DataLoading<Vec<PullRequest>>),
}

pub fn init() -> (Model, Commands<Message>) {
    let repo = "peterstuart/rust-elm-architecture";

    (
        Model {
            repo: repo.into(),
            pull_requests: DataLoading::Loading,
        },
        vec![github::get_pull_requests(repo, |result| {
            Message::ChangePullRequestsDataLoading(
                result.map_or_else(|_| DataLoading::Error, DataLoading::Loaded),
            )
        })],
    )
}

pub fn update(message: Message, model: &mut Model) -> Commands<Message> {
    match message {
        Message::ChangeRepo(repo) => {
            model.repo = repo;
            vec![]
        }
        Message::FetchPullRequests => {
            model.pull_requests = DataLoading::Loading;
            vec![github::get_pull_requests(&model.repo, |result| {
                Message::ChangePullRequestsDataLoading(
                    result.map_or_else(|_| DataLoading::Error, DataLoading::Loaded),
                )
            })]
        }
        Message::ChangePullRequestsDataLoading(data_loading) => {
            model.pull_requests = data_loading;
            vec![]
        }
    }
}

pub fn view(model: &Model) -> Html<Message> {
    Html::div(
        vec![],
        vec![
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
