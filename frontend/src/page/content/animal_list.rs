use yew::{html, Html, function_component};

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
    let litter = &animal.litter;
    vec![
        get_animal_link(&animal.id),
        match litter {
            Some(litter) => litter.id.into(),
            None => "--".into()
        },
        animal.fenotyp.clone().into(),
        match &animal.status {
            AnimalStatus::Alive => "zwierzętarnia".into(),
            AnimalStatus::Dead => "martwy".into(),
            AnimalStatus::Unknown => "nieznany".into(),
            AnimalStatus::Adopted => "adopcja".into()
        },
        match litter {
            Some(litter) => get_animal_link(&litter.father),
            None => "--".into()
        },
        match litter {
            Some(litter) => get_animal_link(&litter.mother),
            None => "--".into()
        }
    ]
}

#[function_component(AnimalList)]
pub fn get_animal_list() -> Html {
    //let animal_list: Html = backend_api::get_all_animal().iter()
    //        .map(animal_to_html).collect();

    let animal_list: Vec<RowProps> = backend_api::get_all_animal().iter()
        .map(animal_to_row).collect();
    let tags = animal_tags();

    html! {
        <>
            <h2>{ "Lista Myszy" }</h2>

            <div id="animal_list">
                <TableWithTags tags={tags} data={animal_list} />
            </div>
        </>
    }
}