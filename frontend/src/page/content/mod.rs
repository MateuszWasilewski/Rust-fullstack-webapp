use yew::{function_component, Html, html};
use yew_router::prelude::{Switch};

mod animal_list;
mod animal;
mod phenotypes;

use animal_list::AnimalList;
use animal::Animal;
use phenotypes::Phenotypes;
use super::routes::Routes;

#[function_component(Content)]
pub fn get_content() -> Html {
    html! {
        <div class="container">
            <Switch<Routes> render={Switch::render(switch)} />
        </div>
    }
}

fn switch(route: &Routes) -> Html {

    match route {
        Routes::Home => html! { <h1>{"Home"} </h1> },
        Routes::List => html! { <AnimalList /> },
        Routes::GoToAnimal{ id } => html! { 
            <Animal animal_id={id.clone()} /> 
        },
        Routes::Phenotypes => html! { <Phenotypes /> }
    } 
}