use yew::MouseEvent;
use yew::{html, function_component, Callback, Html};

use super::Routes;
use yew_router::prelude::use_navigator;
use super::routes::Link;
use super::routes::LinkProps;

#[function_component(NavLink)]
fn get_navlink(props: &LinkProps) -> Html {
    let props = props.clone();
    html! {
        <div class="navbar-nav">
            <Link ..props/>
        </div>
    }
}

#[function_component(Navbar)]
pub fn get_navbar() -> Html {
    let navigator = use_navigator().unwrap();
    let go_home_button = {
        let navigator = navigator.clone();
        let onclick = Callback::from(move |_: MouseEvent|
            navigator.push(&Routes::Home));
        html! {
            <a class="navbar-brand" href="javascript:void(0);" onclick={onclick}>{"Baza Myszy"}</a>
        }   
    };
    
    html! {
        <nav class="navbar navbar-expand-lg bg-light">
            <div class="container-fluid">
                {go_home_button}
                <div class="collapse navbar-collapse" id="navbarSupportedContent">
                    <NavLink target={Routes::AnimalList} link_name={"Lista Myszy"}/>
                    <NavLink target={Routes::Phenotypes} link_name={"Lista Fenotypów"}/>
                    <NavLink target={Routes::AnimalList} link_name={"Lista Miotów"}/>
                </div>
                
                <form class="d-flex mx-auto" role="search">
                    <input class="form-control me-2" type="search" placeholder="Search" aria-label="Search"/>
                    <button class="btn btn-outline-success" type="submit">{"Search"}</button>
                </form>
            </div>
        </nav>
    }
}