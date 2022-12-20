use yew::{html, Html, Component};
use anyhow::Result;

use common::Animal;
use common::animal::AnimalStatus;
use crate::page::routes::AnimalLink;
use crate::common::table::{RowProps, TableWithTags};

fn get_animal_link(id: &str) -> Html {
    html! {
        <AnimalLink id={id.to_owned()} />
    }
}

fn animal_tags() -> RowProps {
    vec! [
        "id osobnika".into(),
        "nr miotu".into(),
        "fenotyp".into(),
        "status".into(),
        "ojciec".into(),
        "matka".into(),
    ]
}

fn animal_to_row(animal: &Animal) -> RowProps {
    let litter = animal.litter.clone().unwrap_or("--".into());
    vec![
        get_animal_link(&animal.id),
        litter.into(),
        animal.fenotyp.clone().into(),
        match &animal.status {
            AnimalStatus::Alive => "zwierzętarnia".into(),
            AnimalStatus::Dead => "martwy".into(),
            AnimalStatus::Unknown => "nieznany".into(),
            AnimalStatus::Adopted => "adopcja".into()
        },
        match &animal.father {
            Some(id) => get_animal_link(&id),
            None => "--".into()
        },
        match &animal.mother {
            Some(id) => get_animal_link(&id),
            None => "--".into()
        }
    ]
}

pub struct AnimalList {
    animals: Option<Vec<Animal>>
}

impl Component for AnimalList {
    type Message = Result<Vec<Animal>>;
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