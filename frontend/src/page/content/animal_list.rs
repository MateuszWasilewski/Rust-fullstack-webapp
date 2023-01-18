use yew::platform::spawn_local;
use yew::{html, Html, function_component};
use yewdux::prelude::{use_store_value, Dispatch};
use yewdux::store::Store;

use crate::common::table::{RowProps, TableWithTags};
use frontend_routing::links::{get_animal_link, get_litter_link};
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

#[derive(PartialEq, Default, Store, Clone)]
struct State {
    animals: Option<Vec<AnimalData>>,
}

async fn fetch_data() {
    let dispatch = Dispatch::<State>::new();
    let state = dispatch.get();
    if let None = state.animals {
        backend_api::get::all_animal().await.map(|animals| {
            dispatch.reduce_mut(|state| state.animals = Some(animals));
        }).ok();
    }
}

#[function_component]
fn Page() -> Html {
    let state = use_store_value::<State>();
    let animal_list: Vec<RowProps> = state.animals.clone().unwrap_or_default().iter()
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

#[function_component]
pub fn AnimalList() -> Html {
    spawn_local(fetch_data());

    html! {
        <Page />
    }
}
