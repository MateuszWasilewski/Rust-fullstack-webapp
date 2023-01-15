use anyhow::Result;
use yew::{html, Component, Html};

use crate::common::table::{RowProps, TableWithTags};
use crate::page::routes::{get_animal_link, get_litter_link};
use common::AnimalData;

fn animal_tags() -> RowProps {
    vec![
        "id osobnika".into(),
        "nr miotu".into(),
        "fenotyp".into(),
        "status".into(),
        "ojciec".into(),
        "matka".into(),
    ]
}

fn animal_to_row(animal: &AnimalData) -> RowProps {
    vec![
        get_animal_link(&animal.id),
        match &animal.litter {
            Some(id) => get_litter_link(&id),
            None => "--".into()
        },
        animal.fenotyp.clone().into(),
        animal.status.clone().unwrap_or("nieznany".into()).into(),
        match &animal.father {
            Some(id) => get_animal_link(&id),
            None => "--".into(),
        },
        match &animal.mother {
            Some(id) => get_animal_link(&id),
            None => "--".into(),
        },
    ]
}

pub struct AnimalList {
    animals: Option<Vec<AnimalData>>,
}

impl Component for AnimalList {
    type Message = Result<Vec<AnimalData>>;
    type Properties = ();

    fn create(ctx: &yew::Context<Self>) -> Self {
        ctx.link().send_future({
            async move {
                let animals = backend_api::get_all_animal().await;
                animals
            }
        });
        AnimalList { animals: None }
    }

    fn update(&mut self, _ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        self.animals = msg.ok();
        true
    }

    fn view(&self, _ctx: &yew::Context<Self>) -> Html {
        html! {
        <>
        <h2>{ "Lista Myszy" }</h2>
        {match &self.animals {
            None => html!{},
            Some(animals) => {
                let animal_list: Vec<RowProps> = animals.iter()
                    .map(animal_to_row).collect();
                let tags = animal_tags();
                html! {
                    <div id="animal_list">
                        <TableWithTags tags={tags} data={animal_list} />
                    </div>
                }
            },
        }}
        </>
        }
    }
}
