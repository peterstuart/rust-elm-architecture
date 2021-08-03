use rust_elm_architecture::{Attribute, Commands, Html};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Model {
    counter: i32,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Message {
    Increment,
    Decrement,
}

pub fn init() -> (Model, Commands<Message>) {
    (Model { counter: 0 }, vec![])
}

pub fn update(message: Message, model: &mut Model) -> Commands<Message> {
    match message {
        Message::Increment => model.counter += 1,
        Message::Decrement => model.counter -= 1,
    };

    vec![]
}

pub fn view(model: &Model) -> Html<Message> {
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
    )
}
