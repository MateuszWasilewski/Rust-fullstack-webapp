use yew::MouseEvent;
use yew::prelude::{html, Html, function_component, Callback};
use yew_router::prelude::{use_history, AnyHistory};
use yew_router::history::History;

use super::Routes;

fn get_link(history: &AnyHistory, target: Routes, name: &str) -> Html {
    let history = history.clone();
    let onclick = Callback::once(move |_: MouseEvent| history.push(target));
    html! {
        <a class="nav-link active" href="javascript:void(0);" onclick={onclick}>{name}</a>
    }
}

#[function_component(Navbar)]
pub fn get_navbar() -> Html {
    let history = use_history().unwrap();
    let go_home_button = {
        let history = history.clone();
        let onclick = Callback::once(move |_: MouseEvent|
            history.push(Routes::Home));
        html! {
            <a class="navbar-brand" href="javascript:void(0);" onclick={onclick}>{"Animal Database"}</a>
        }   
    };
    let go_to_animal_list = get_link(&history, Routes::List, "Animal List");

    html! {
        <nav class="navbar navbar-expand-lg bg-light">
            <div class="container-fluid">
                {go_home_button}
                <div class="collapse navbar-collapse" id="navbarSupportedContent">
                    {go_to_animal_list}
                </div>
            </div>
        </nav>
    }
}