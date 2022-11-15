extern crate yew;

mod page;
mod wasm;

use wasm::log as log;

fn main() {
    yew::start_app::<page::App>();
}