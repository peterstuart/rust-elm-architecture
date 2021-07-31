mod app;
mod renderer;
mod virtual_dom;

use app::App;
use virtual_dom::{Attribute, Html};
use wasm_bindgen::prelude::*;

#[derive(Clone, Debug, Eq, PartialEq)]
struct Model {
    counter: i32,
    text: String,
}

impl Model {
    fn increment(&self) -> Self {
        Self {
            counter: self.counter + 1,
            text: self.text.clone(),
        }
    }

    fn decrement(&self) -> Self {
        Self {
            counter: self.counter - 1,
            text: self.text.clone(),
        }
    }

    fn change_text(&self, text: &str) -> Self {
        Self {
            counter: self.counter,
            text: text.into(),
        }
    }
}

impl Default for Model {
    fn default() -> Self {
        Self {
            counter: 0,
            text: "".into(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum Message {
    Increment,
    Decrement,
    ChangeText(String),
}

fn init() -> Model {
    Model::default()
}

fn update(message: &Message, model: &Model) -> Model {
    match message {
        Message::Increment => model.increment(),
        Message::Decrement => model.decrement(),
        Message::ChangeText(text) => model.change_text(text),
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
                    Attribute::value(&model.text),
                    Attribute::on_input(|text| Message::ChangeText(text.into())),
                ],
                vec![],
            ),
            Html::text(&model.text),
        ],
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
