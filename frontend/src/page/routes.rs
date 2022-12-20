use yew::{html, Html, Callback, MouseEvent, function_component, Properties, AttrValue};
use yew_router::prelude::*;

#[derive(Debug, Clone, PartialEq, Routable)]
pub enum Routes {
    #[at("/")]
    Home,
    #[at("/animal-list")]
    AnimalList,
    #[at("/animal/:id")]
    GoToAnimal {id: String},
    #[at("/phenotypes")]
    Phenotypes,
    #[at("/litter-list")]
    Litters,
    #[at("/add/*")]
    Add
}

#[derive(PartialEq, Properties, Clone)]
pub struct LinkProps<T: Routable> {
    pub target: T,
    pub link_name: String
}

#[function_component(Link)]
pub fn get_link<T: Routable + 'static>(props: &LinkProps<T>) -> Html {
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

#[function_component]
pub fn AnimalLink(props: &AnimalLinkProps) -> Html {
    let onclick = {
        let navigator = use_navigator().unwrap();
        let id = props.id.clone();
        Callback::from(move |_: MouseEvent| {
        navigator.push(&Routes::GoToAnimal { id: id.to_string() })
    })};
    html! {
        <a class="nav-link active text-primary" href="javascript:void(0);" {onclick}>{&props.id}</a>
    }
}