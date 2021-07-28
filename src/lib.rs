mod app;
mod utils;
mod virtual_dom;

use app::App;
use virtual_dom::{Attribute, Html};
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, rust-elm-architecture!");
}

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

#[derive(Clone, Debug)]
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
            Html::text(&model.counter.to_string()),
            Html::button(
                &[Attribute::on_click(&Message::Increment)],
                &[Html::text("+")],
            ),
        ],
    )
}

#[wasm_bindgen]
pub fn run_app() {
    let app = App { init, update, view };
    app.run()
}
