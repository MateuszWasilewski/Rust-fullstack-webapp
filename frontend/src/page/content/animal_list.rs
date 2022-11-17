use yew::{html, Component, Context, Html};

use common::Animal;
//use crate::page::{App, Msg};

pub struct AnimalList {
    all_animal: Vec<Animal>,
    filtered_animals: Vec<Animal>
}

fn animal_tags() -> Html {
    html! {
        <div class="row fst-italic">
            <div class="col">
                { "id osobnika" }
            </div>
            <div class="col">
                { "linia" }
            </div>
            <div class="col">
                { "data urodzenia" }
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
                { format!("{}", animal.id) }
            </div>
            <div class="col">
                { format!("{}", animal.id) }
            </div>
            <div class="col">
                { format!("{}", animal.id) }
            </div>
            <div class="col">
                { format!("{}", animal.id) }
            </div>
            <div class="col">
                { format!("{}", animal.id) }
            </div>
            <div class="col">
                { format!("{}", animal.id) }
            </div>
        </div>
    }
}

impl Component for AnimalList {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let animal  = AnimalList { 
            all_animal: backend_api::get_all_animal(),
            filtered_animals: backend_api::get_all_animal()
        };
        animal
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        self.all_animal.len() != self.filtered_animals.len()
    }

    fn view(&self, _ctx: &Context<Self>) -> yew::Html {
        let animal_list: Html = self.filtered_animals.iter()
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
}