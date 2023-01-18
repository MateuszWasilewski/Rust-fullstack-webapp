use yew::{function_component, html, Html};
use yew_router::prelude::Switch;

mod animal;
mod litter;
mod phenotype;

use animal::AddAnimalTemp as AddAnimal;
use litter::AddLitter;
use phenotype::AddPhenotype;
use frontend_routing::RoutesAdd;

#[function_component(AddContent)]
pub fn get_add_content() -> Html {
    html! {
        <Switch<RoutesAdd> render={switch} />
    }
}

fn switch(route: RoutesAdd) -> Html {
    match route {
        RoutesAdd::Litter => html! { <AddLitter /> },
        RoutesAdd::Animal => html! { <AddAnimal /> },
        RoutesAdd::Phenotype => html! { <AddPhenotype /> },
    }
}
