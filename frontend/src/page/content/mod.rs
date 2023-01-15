use yew::{function_component, html, Html};
use yew_router::prelude::Switch;

pub mod add;
mod animal;
mod animal_list;
mod litter_list;
mod phenotypes;
mod litter_page;

use super::routes::Routes;
use add::AddContent;
use animal::Animal;
use animal_list::AnimalList;
use litter_list::LitterList;
use phenotypes::Phenotypes;
use litter_page::LitterPage;

#[function_component(Content)]
pub fn get_content() -> Html {
    html! {
        <div class="container">
            <Switch<Routes> render={switch} />
        </div>
    }
}

fn switch(route: Routes) -> Html {
    match route {
        Routes::Home => html! { <h1>{"Home"} </h1> },
        Routes::AnimalList => html! { <AnimalList /> },
        Routes::GoToAnimal { id } => html! {
            <Animal animal_id={id} />
        },
        Routes::GoToLitter { id } => html! {
            <LitterPage {id} />
        },
        Routes::Phenotypes => html! { <Phenotypes /> },
        Routes::Litters => html! { <LitterList /> },
        Routes::Add => html! { <AddContent /> },
    }
}
