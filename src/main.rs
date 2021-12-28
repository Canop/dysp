mod app;
mod editor;
pub mod dom;
mod lang;

pub use {
    app::App,
    dom::{Append, DomElement},
    editor::*,
    lang::*,
};
pub use {
    gloo::{
        console::*,
        events::*,
        utils::*,
    },
    web_sys::*,
    wasm_bindgen::{
        prelude::*,
        JsCast,
    },
};

static mut APP: Option<App> = None;

fn set_app(app: App) {
    unsafe {
        APP = Some(app);
    }
}
pub fn app() -> &'static mut App {
    unsafe {
        APP.as_mut().unwrap()
    }
}

fn main() {
    console_error_panic_hook::set_once();
    log!("test from main");
    let mut app = App::new(&mut body()).expect("not to crash");
    app.on_code_change().unwrap();
    set_app(app);
}
