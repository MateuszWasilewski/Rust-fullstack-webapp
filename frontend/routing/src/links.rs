use yew::{Properties, function_component, Html, Callback, MouseEvent, html, AttrValue};
use yew_router::{Routable, prelude::use_navigator};

use super::Routes;

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