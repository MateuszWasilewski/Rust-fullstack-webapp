use yew::{function_component, html, AttrValue, Callback, Html, MouseEvent, Properties};
use yew_router::prelude::*;

#[derive(Debug, Clone, PartialEq, Routable)]
pub enum Routes {
    #[at("/")]
    Home,
    #[at("/animal-list")]
    AnimalList,
    #[at("/animal/:id")]
    GoToAnimal { id: String },
    #[at("/litter/:id")]
    GoToLitter { id: String },
    #[at("/phenotypes")]
    Phenotypes,
    #[at("/litter-list")]
    Litters,
    #[at("/add/*")]
    Add,
    #[at("/search/:query")]
    Search { query: String }
}

#[derive(PartialEq, Properties, Clone)]
pub struct LinkProps<T: Routable> {
    pub target: T,
    pub link_name: String,
}

#[function_component]
pub fn Link<T: Routable + 'static>(props: &LinkProps<T>) -> Html {
    let navigator = use_navigator().unwrap();
    let route = props.target.clone();
    let onclick = Callback::from(move |_: MouseEvent| navigator.push(&route));
    html! {
        <a class="nav-link active" href="javascript:void(0);" onclick={onclick}>{props.link_name.to_owned()}</a>
    }
}

pub fn get_animal_link(id: &str) -> Html {
    html! {
        <AnimalLink id={id.to_owned()} />
    }
}

#[derive(PartialEq, Properties)]
pub struct AnimalLinkProps {
    pub id: AttrValue,
}

#[function_component]
pub fn AnimalLink(props: &AnimalLinkProps) -> Html {
    let onclick = {
        let navigator = use_navigator().unwrap();
        let id = props.id.clone();
        Callback::from(move |_: MouseEvent| {
            navigator.push(&Routes::GoToAnimal { id: id.to_string() })
        })
    };
    html! {
        <a class="nav-link active text-primary" href="javascript:void(0);" {onclick}>{&props.id}</a>
    }
}

pub fn get_litter_link(id: &str) -> Html {
    html! {
        <LitterLink id={id.to_owned()} />
    }
}

#[derive(PartialEq, Properties)]
pub struct LitterLinkProps {
    pub id: AttrValue,
}

#[function_component]
pub fn LitterLink(props: &LitterLinkProps) -> Html {
    let onclick = {
        let navigator = use_navigator().unwrap();
        let id = props.id.clone();
        Callback::from(move |_| navigator.push(&Routes::GoToLitter { id: id.to_string() }))
    };
    html! {
        <a class="nav-link active text-primary" href="javascript:void(0);" {onclick}>{&props.id}</a>
    }
}