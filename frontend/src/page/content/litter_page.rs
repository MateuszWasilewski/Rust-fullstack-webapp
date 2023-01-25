use common::AnimalData;
use frontend_routing::Routes;
use web_sys::MouseEvent;
use yew::platform::spawn_local;
use yew::{function_component, html, Html, Properties, Callback};
use yew_router::prelude::use_navigator;
use yewdux::dispatch::Dispatch;
use yewdux::prelude::use_store_value;
use yewdux::store::Store;

use crate::common::table::{RowProps, TableWithTags};
use frontend_routing::links::{get_animal_link, get_litter_link};

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
    let animals = backend_api::get::animal_litter_list(&state.litter_id).await;
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
    let navigator = use_navigator().unwrap();
    let state = use_store_value::<PageState>();
    let animal_list = &state.animals;

    match animal_list.is_empty() {
        false => {
            let animal_list: Vec<RowProps> = animal_list.iter().map(animal_to_row).collect();

            html! {
            <>
                <h2>{format!("Informacje o miocie: {}", state.litter_id)}</h2>
                <div id="animal_list">
                    <TableWithTags tags={animal_tags()} data={animal_list} />
                </div>
            </>
            }
        },
        true => {
            let id = state.litter_id.clone();
            let onclick = Callback::from({
                move |_: MouseEvent| {
                    let navigator = navigator.clone();
                    let id = id.clone();
                    spawn_local(async move {
                        match backend_api::delete::litter(&id).await {
                            Ok(_) => navigator.push(&Routes::Home),
                            Err(_) => navigator.push(&Routes::ServerError)
                        }
                    });
                }
            });
            html! {
            <>
                <h2>{format!("Informacje o miocie: {}", state.litter_id)}</h2>
                <h4>{"Miot jest pusty"}</h4>
                <button type="button" class="btn btn-primary btn-sm" {onclick}>{"Usuń miot"}</button>
            </>
            }
        }
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
