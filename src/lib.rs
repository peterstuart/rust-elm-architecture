mod app;
mod virtual_dom;

use app::App;
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
