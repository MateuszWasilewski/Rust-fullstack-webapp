use common::AnimalData;
use yew::platform::spawn_local;
use yew::{function_component, html, Html, Properties};
use yewdux::dispatch::Dispatch;
use yewdux::prelude::use_selector;
use yewdux::store::Store;

use crate::common::table::{RowProps, TableWithTags};
use crate::page::routes::{get_animal_link, get_litter_link};

#[derive(PartialEq, Properties)]
pub struct LitterPageProps {
    pub id: String,
}

#[derive(PartialEq, Default, Store, Clone)]
struct PageState {
    litter_id: String,
    animals: Vec<AnimalData>,
}

async fn fetch_data(dispatch: Dispatch<PageState>) {
    let state = dispatch.get();
    let animals = backend_api::get_animal_litter_list(&state.litter_id).await;
    if let Ok(animals) = animals {
        dispatch.reduce_mut(|state| state.animals = animals);
    }
}

fn animal_tags() -> RowProps {
    vec![
        "id osobnika".into(),
        "nr miotu".into(),
        "fenotyp".into(),
        "status".into(),
    ]
}

fn animal_to_row(animal: &AnimalData) -> RowProps {
    let litter = animal.litter.clone().unwrap_or_default();
    vec![
        get_animal_link(&animal.id),
        get_litter_link(&litter),
        animal.fenotyp.clone().into(),
        animal.status.clone().unwrap_or("nieznany".into()).into(),
    ]
}

#[function_component]
fn Page() -> Html {
    let animal_list = use_selector(|state: &PageState| state.animals.clone());

    let animal_list: Vec<RowProps> = animal_list.iter().map(animal_to_row).collect();

    html! {
      <div id="animal_list">
        <TableWithTags tags={animal_tags()} data={animal_list} />
      </div>
    }
}

#[function_component]
pub fn LitterPage(props: &LitterPageProps) -> Html {
    let dispatch = Dispatch::<PageState>::new();
    dispatch.reduce_mut(|state| state.litter_id = props.id.clone());
    spawn_local(fetch_data(dispatch.clone()));

    html! {
      <Page />
    }
}
