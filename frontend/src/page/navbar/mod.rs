use yew::MouseEvent;
use yew::{function_component, html, Callback, Html};
use yew_router::Routable;
use yew_router::prelude::use_navigator;

mod search;

use super::content::add::routes::Routes as AddRoutes;
use super::Routes;
use search::SearchBar;
use super::routes::Link;
use super::routes::LinkProps;

#[function_component]
fn NavLink<T: Routable + 'static>(props: &LinkProps<T>) -> Html {
    let props = props.clone();
    html! {
        <li class="nav-item dropdown">
            <Link<T> ..props/>
        </li>
    }
}

#[function_component]
fn DropdownLink<T: Routable + 'static>(props: &LinkProps<T>) -> Html {
    let props = props.clone();
    html! {
        <li class="dropdown-item">
            <Link<T> ..props/>
        </li>
    }
}

#[function_component(Navbar)]
pub fn get_navbar() -> Html {
    let navigator = use_navigator().unwrap();
    let go_home_button = {
        let navigator = navigator.clone();
        let onclick = Callback::from(move |_: MouseEvent| navigator.push(&Routes::Home));
        html! {
            <a class="navbar-brand" href="javascript:void(0);" onclick={onclick}>{"Baza Myszy"}</a>
        }
    };

    html! {
        <nav class="navbar navbar-expand-lg bg-light">
            <div class="container-fluid">
                {go_home_button}
                <button class="navbar-toggler" type="button" data-bs-toggle="collapse" data-bs-target="#navbarSupportedContent" aria-controls="navbarNav" aria-expanded="false" aria-label="Toggle navigation">
                    <span class="navbar-toggler-icon"></span>
                </button>
                <div class="collapse navbar-collapse" id="navbarSupportedContent">
                <ul class="navbar-nav">
                    <NavLink<Routes> target={Routes::AnimalList} link_name={"Lista Myszy"}/>
                    <NavLink<Routes> target={Routes::Phenotypes} link_name={"Lista Fenotypów"}/>
                    <NavLink<Routes> target={Routes::Litters} link_name={"Lista Miotów"}/>
                    <li class="nav-item dropdown">
                    <div class="btn-group">
                        <a class="nav-link dropdown-toggle" role="button" data-bs-toggle="dropdown" aria-expanded="false">
                            {"Dodaj"}
                        </a>
                        <ul class="dropdown-menu">
                            <DropdownLink<AddRoutes> target={AddRoutes::Litter} link_name={"Miot"}/>
                            <DropdownLink<AddRoutes> target={AddRoutes::Animal} link_name={"Mysz"}/>
                            <DropdownLink<AddRoutes> target={AddRoutes::Phenotype} link_name={"Fenotyp"}/>
                        </ul>
                    </div>
                    </li>
                    <SearchBar/>
                </ul>
                </div>
            </div>
        </nav>
    }
}
