extern crate yew;
use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
            <h1>{ "Hello World" }</h1>
            <h2>{ "More text" }</h2>
        </>
    }
}

fn main() {
    yew::start_app::<App>();
}