use yew::Renderer;

mod page;
mod common;

fn main() {
    Renderer::<page::App>::new().render();
}