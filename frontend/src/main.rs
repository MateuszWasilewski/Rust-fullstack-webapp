extern crate yew;

mod page;
mod wasm;
mod common;

fn main() {
    yew::start_app::<page::App>();
}