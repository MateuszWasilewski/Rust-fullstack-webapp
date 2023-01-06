use yew::Renderer;

mod common;
mod page;

fn main() {
    Renderer::<page::App>::new().render();
}
