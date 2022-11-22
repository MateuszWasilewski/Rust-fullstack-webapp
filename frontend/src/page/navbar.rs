use yew::MouseEvent;
use yew::{html, function_component, Callback, Html};
use yew_router::prelude::{use_history};
use yew_router::history::History;

use super::Routes;
use super::routes::get_link;

#[function_component(Navbar)]
pub fn get_navbar() -> Html {
    let history = use_history().unwrap();
    let go_home_button = {
        let history = history.clone();
        let onclick = Callback::once(move |_: MouseEvent|
            history.push(Routes::Home));
        html! {
            <a class="navbar-brand" href="javascript:void(0);" onclick={onclick}>{"Baza Myszy"}</a>
        }   
    };
    let go_to_animal_list = get_link(Routes::List, "Lista Myszy");
    let go_to_phenotypes = get_link(Routes::Phenotypes, "Lista Fenotypów");
    let go_to_litters = get_link(Routes::Phenotypes, "Lista Miotów");


    let navbar_options = vec! [
        html! {go_to_animal_list},
        html! {go_to_phenotypes},
        html! {go_to_litters}
    ].into_iter().map(|nav_link| {
        html! {
            <>
                <div class="navbar-nav">
                    {nav_link}
                </div>
            </>
        }
    } ).collect::<Html>();

    html! {
        <nav class="navbar navbar-expand-lg bg-light">
            <div class="container-fluid">
                {go_home_button}
                <div class="collapse navbar-collapse" id="navbarSupportedContent">
                    {navbar_options}
                </div>
                
                <form class="d-flex mx-auto" role="search">
                    <input class="form-control me-2" type="search" placeholder="Search" aria-label="Search"/>
                    <button class="btn btn-outline-success" type="submit">{"Search"}</button>
                </form>
            </div>
        </nav>
    }
}