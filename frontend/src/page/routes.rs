use yew_router::prelude::*;
use yew::{html, Html, Callback, MouseEvent};

#[derive(Debug, Clone, PartialEq, Routable)]
pub enum Routes {
    #[at("/")]
    Home,
    #[at("/animal-list")]
    List,
    #[at("/animal/:id")]
    GoToAnimal {id: String},
    #[at("/phenotypes")]
    Phenotypes
}


pub fn get_link(target: Routes, name: &str) -> Html {
    let history = use_history().unwrap();
    let onclick = Callback::once(move |_: MouseEvent| history.push(target));
    html! {
        <a class="nav-link active" href="javascript:void(0);" onclick={onclick}>{name}</a>
    }
}

pub fn get_animal_link(name: &str) -> Html {
    let animal = backend_api::get_animal_by_id(name);
    match animal {
        Some(_) => {
            let history = use_history().unwrap();
            let onclick = {
                let history = history.clone();
                let name = name.to_owned();
                Callback::once(move |_: MouseEvent| {
                history.push(Routes::GoToAnimal { id: name })
            })};
            html! {
                <a class="nav-link active text-primary" href="javascript:void(0);" onclick={onclick}>{name}</a>
            }
        }
        None => {
            html! {
                {name}
            }
        }
    }
}