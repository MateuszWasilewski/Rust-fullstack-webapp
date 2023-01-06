use yew::{function_component, html, Html};
use yew_router::prelude::Switch;

mod animal;
mod litter;
mod phenotype;
pub mod routes;

use animal::AddAnimalTemp as AddAnimal;
use litter::AddLitter;
use phenotype::AddPhenotype;
use routes::Routes;

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
        Routes::Phenotype => html! { <AddPhenotype /> },
    }
}
