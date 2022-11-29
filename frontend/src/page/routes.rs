use yew_router::prelude::*;
use yew::{html, Html, Callback, MouseEvent, function_component, Properties, AttrValue};

#[derive(Debug, Clone, PartialEq, Routable)]
pub enum Routes {
    #[at("/")]
    Home,
    #[at("/animal-list")]
    AnimalList,
    #[at("/animal/:id")]
    GoToAnimal {id: String},
    #[at("/phenotypes")]
    Phenotypes
}

#[derive(PartialEq, Properties, Clone)]
pub struct LinkProps {
    pub target: Routes,
    pub link_name: String
}

#[function_component(Link)]
pub fn get_link(props: &LinkProps) -> Html {
    let navigator = use_navigator().unwrap();
    let route = props.target.clone();
    let onclick = Callback::from(move |_: MouseEvent| navigator.push(&route));
    html! {
        <a class="nav-link active" href="javascript:void(0);" onclick={onclick}>{props.link_name.to_owned()}</a>
    }
}

#[derive(PartialEq, Properties)]
pub struct AnimalLinkProps {
    pub id: AttrValue
}

#[function_component(AnimalLink)]
pub fn get_animal_link(props: &AnimalLinkProps) -> Html {
    let animal = backend_api::get_animal_by_id(&props.id);
    let navigator = use_navigator().unwrap();
    let id = props.id.clone();
    match animal {
        Some(_) => {
            let onclick = {
                let navigator = navigator.clone();
                let id = id.clone();
                Callback::from(move |_: MouseEvent| {
                navigator.push(&Routes::GoToAnimal { id: id.to_string() })
            })};
            html! {
                <a class="nav-link active text-primary" href="javascript:void(0);" onclick={onclick}>{id}</a>
            }
        }
        None => {
            html! {
                {id}
            }
        }
    }
}