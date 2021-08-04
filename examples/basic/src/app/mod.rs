mod counter;
mod data_loading;
mod github;

pub use data_loading::DataLoading;

use rust_elm_architecture::{command, App, Commands, Html};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Model {
    counter: counter::Model,
    github: github::Model,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Message {
    Counter(counter::Message),
    Github(github::Message),
}

fn init() -> (Model, Commands<Message>) {
    let mut commands = vec![];

    let (counter, counter_commands) = counter::init();
    commands.append(
        &mut counter_commands
            .into_iter()
            .map(|command| command::map(command, Message::Counter))
            .collect(),
    );

    let (github, github_commands) = github::init();
    commands.append(
        &mut github_commands
            .into_iter()
            .map(|command| command::map(command, Message::Github))
            .collect(),
    );

    let model = Model { counter, github };

    (model, commands)
}

fn update(message: Message, model: &mut Model) -> Commands<Message> {
    match message {
        Message::Counter(message) => counter::update(message, &mut model.counter)
            .into_iter()
            .map(|command| command::map(command, Message::Counter))
            .collect(),
        Message::Github(message) => github::update(message, &mut model.github)
            .into_iter()
            .map(|command| command::map(command, Message::Github))
            .collect(),
    }
}

fn view(model: &Model) -> Html<Message> {
    Html::div(
        vec![],
        vec![
            counter::view(&model.counter).map(Message::Counter),
            github::view(&model.github).map(Message::Github),
        ],
    )
}

pub fn create(root_id: &str) -> App<Model, Message> {
    App::new(init, update, view, root_id)
}
