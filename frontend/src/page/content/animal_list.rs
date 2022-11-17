use yew::{html, Html, function_component};

use common::Animal;
use crate::page::routes::get_blue_link;
use crate::page::Routes;

fn animal_tags() -> Html {
    html! {
        <div class="row fst-italic">
            <div class="col">
                { "id osobnika" }
            </div>
            <div class="col">
                { "nr miotu" }
            </div>
            <div class="col">
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

fn animal_to_html(animal: &Animal) -> Html{
    html! {
        <div class="row border-top">
            <div class="col">
                {get_blue_link(Routes::GoToAnimal{id: animal.id.clone()}, &animal.id)}
            </div>
            <div class="col">
                { format!("{}", animal.miot) }
            </div>
            <div class="col">
                { format!("{}", animal.fenotyp) }
            </div>
            <div class="col">
                { format!("{:?}", animal.status) }
            </div>
            <div class="col">
                { format!("{}", animal.father) }
            </div>
            <div class="col">
                { format!("{}", animal.mother) }
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
            <h2>{ "Start Animal List" }</h2>

            <div id="animal_list">
                {animal_tags()}
                {animal_list}
            </div>
        </>
    }
}