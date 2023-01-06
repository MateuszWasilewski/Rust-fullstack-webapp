use yew::{function_component, html, Html};
use yew_router::prelude::Switch;

pub mod routes;
mod litter;
mod animal;
mod phenotype;

use routes::Routes;
use litter::AddLitter;
use animal::AddAnimalTemp as AddAnimal;
use phenotype::AddPhenotype;

#[function_component(AddContent)]
pub fn get_add_content() -> Html {
    html! {
        <Switch<Routes> render={switch} />
    }
}

fn switch(route: Routes) -> Html {
    match route {
        Routes::Litter => html! { <AddLitter /> },
        Routes::Animal => html! { <AddAnimal /> },
        Routes::Phenotype => html! { <AddPhenotype /> }
    }
}