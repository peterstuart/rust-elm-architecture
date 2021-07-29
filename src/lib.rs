mod app;
mod utils;
mod virtual_dom;

use app::App;
use utils::set_panic_hook;
use virtual_dom::{Attribute, Html};
use wasm_bindgen::prelude::*;

#[derive(Clone, Debug, Eq, PartialEq)]
struct Model {
    counter: i32,
}

impl Model {
    fn increment(&self) -> Self {
        Self {
            counter: self.counter + 1,
        }
    }

    fn decrement(&self) -> Self {
        Self {
            counter: self.counter - 1,
        }
    }
}

impl Default for Model {
    fn default() -> Self {
        Self { counter: 0 }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum Message {
    Increment,
    Decrement,
}

fn init() -> Model {
    Model::default()
}

fn update(message: &Message, model: &Model) -> Model {
    match message {
        Message::Increment => model.increment(),
        Message::Decrement => model.decrement(),
    }
}

fn view(model: &Model) -> Html<Message> {
    Html::div(
        &[],
        &[
            Html::button(
                &[Attribute::on_click(&Message::Decrement)],
                &[Html::text("-")],
            ),
            Html::span(
                &[Attribute::class("counter")],
                &[Html::text(&model.counter.to_string())],
            ),
            Html::button(
                &[Attribute::on_click(&Message::Increment)],
                &[Html::text("+")],
            ),
        ],
    )
}

#[wasm_bindgen]
pub fn run_app() {
    set_panic_hook();

    let app = App::new(init, update, view, "root");
    app.start();
}
