use yew::MouseEvent;
use yew::prelude::{html, function_component, Callback};
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
            <a class="navbar-brand" href="javascript:void(0);" onclick={onclick}>{"Animal Database"}</a>
        }   
    };
    let go_to_animal_list = get_link(Routes::List, "Animal List");

    let go_to_selected_animal = get_link(Routes::GoToAnimal { id: "66.F3".to_owned() }, "Selected animal");

    html! {
        <nav class="navbar navbar-expand-lg bg-light">
            <div class="container-fluid">
                {go_home_button}
                <div class="collapse navbar-collapse" id="navbarSupportedContent">
                    <div class="navbar-nav">
                        {go_to_animal_list}
                        {go_to_selected_animal}
                    </div>
                </div>
            </div>
        </nav>
    }
}