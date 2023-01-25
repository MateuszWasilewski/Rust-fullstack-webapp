use yew::{function_component, html, Html};
use yew_router::prelude::Switch;

pub mod add;
mod animal;
mod animal_list;
mod litter_list;
mod phenotypes;
mod litter_page;
mod search_results;
mod error;
mod home;
mod not_found;

use add::AddContent;
use animal::Animal;
use animal_list::AnimalList;
use litter_list::LitterList;
use phenotypes::Phenotypes;
use litter_page::LitterPage;
use search_results::SearchResults;
use frontend_routing::Routes;
use home::Home;
use not_found::NotFound;
use error::Error;

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
        Routes::Home => html! { <Home/> },
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
        Routes::Search { query } => html ! { <SearchResults {query} />},
        Routes::NotFound => html! { <NotFound /> },
        Routes::ServerError => html! { <Error /> }
    }
}
