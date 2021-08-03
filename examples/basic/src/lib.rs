mod app;
mod github;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn run_app() {
    init_log();
    set_panic_hook();

    let app = app::create("root");
    app.start();
}

fn init_log() {
    #[cfg(feature = "console_log")]
    {
        use log::Level;
        console_log::init_with_level(Level::Trace).expect("error initializing log");
    }
}

fn set_panic_hook() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}
