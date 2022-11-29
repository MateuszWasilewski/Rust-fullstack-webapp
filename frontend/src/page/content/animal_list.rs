use yew::{html, Html, function_component};

use common::Animal;
use common::animal::AnimalStatus;
use crate::page::routes::AnimalLink;

fn get_animal_link(id: &str) -> Html {
    html! {
        <AnimalLink id={id.to_owned()} />
    }
}

fn animal_tags() -> Html {
    html! {
        <div class="row fst-italic">
            <div class="col">
                { "id osobnika" }
            </div>
            <div class="col">
                { "nr miotu" }
            </div>
            <div class="col-3">
                { "fenotyp" }
            </div>
            <div class="col">
                { "status" }
            </div>
            <div class="col">
                { "ojciec" }
            </div>
            <div class="col">
                { "matka" }
            </div>
        </div>
    }
}

fn animal_to_html(animal: &Animal) -> Html {
    let litter = &animal.litter;

    html! {
        <div class="row border-top">
            <div class="col">
                {get_animal_link(&animal.id)}
            </div>
            <div class="col">
                { format!("{}", match litter {
                    Some(litter) => litter.id.to_string(),
                    None => "--".to_owned()
                }) }
            </div>
            <div class="col-3">
                { format!("{}", animal.fenotyp) }
            </div>
            <div class="col">
                { match &animal.status {
                    AnimalStatus::Alive => "zwierzętarnia",
                    AnimalStatus::Dead => "martwy",
                    AnimalStatus::Unknown => "nieznany",
                    AnimalStatus::Adopted => "adopcja"
                } }
            </div>
            <div class="col">
                { match litter {
                    Some(litter) => get_animal_link(&litter.father),
                    None => html! { {{"--".to_owned()}} }
                } }
            </div>
            <div class="col">
                { match litter {
                    Some(litter) => get_animal_link(&litter.mother),
                    None => html! { {{"--".to_owned()}} }
                } }
            </div>
        </div>
    }
}

#[function_component(AnimalList)]
pub fn get_animal_list() -> Html {
    let animal_list: Html = backend_api::get_all_animal().iter()
            .map(animal_to_html).collect();

    html! {
        <>
            <h2>{ "Lista Myszy" }</h2>

            <div id="animal_list">
                {animal_tags()}
                {animal_list}
            </div>
        </>
    }
}