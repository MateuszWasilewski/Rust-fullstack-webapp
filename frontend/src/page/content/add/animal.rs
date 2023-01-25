use common::animal::Gender;
use common::AnimalData;
use frontend_routing::Routes;
use yew::platform::spawn_local;
use yew::{function_component, html, Callback, Html};
use yew_router::prelude::use_navigator;
use yewdux::dispatch::Dispatch;
use yewdux::prelude::use_selector;
use yewdux::store::Store;

use crate::common::input::event_to_text;
use crate::common::input::DropdownForm;
use crate::common::input::TextInput;
use anyhow::Result;
use std::rc::Rc;
use wasm_utils::log;

async fn get_phenotypes() -> Result<Vec<String>> {
    Ok(backend_api::get::phenotypes()
        .await?
        .into_iter()
        .map(|phen| phen.phenotype)
        .collect())
}

async fn get_litters() -> Result<Vec<String>> {
    Ok(backend_api::get::litter_list()
        .await?
        .into_iter()
        .map(|litter| litter.id)
        .collect())
}

#[derive(Default, PartialEq, Clone, Store)]
struct State {
    litters: Vec<String>,
    phenotypes: Vec<String>,
    id: Option<String>,
    litter: Option<String>,
    phenotype: Option<String>,
    gender: Option<Gender>,
    status: Option<String>,
    eye_color: Option<String>,
    hair: Option<String>
}

async fn set_litters(dispatch: Dispatch<State>) {
    let mut litters = get_litters().await.unwrap_or_default();
    litters.sort();
    dispatch.reduce_mut(|state| state.litters = litters)
}

async fn set_phenotypes(dispatch: Dispatch<State>) {
    let mut phenotypes = get_phenotypes().await.unwrap_or_default();
    phenotypes.sort();
    dispatch.reduce_mut(|state| state.phenotypes = phenotypes)
}

#[function_component]
fn SelectLitter() -> Html {
    let litters = use_selector(|state: &State| state.litters.clone());
    let dispatch = Dispatch::<State>::new();
    let set_value = Callback::from(move |value: Option<String>| {
        dispatch.reduce_mut(|state| state.litter = value)
    });
    html! {
        <DropdownForm {set_value} id={"litter"} text={"Wybierz miot"} options={litters} default={"Nieznany"} />
    }
}

#[function_component]
fn SelectPhenotype() -> Html {
    let phenotypes = use_selector(|state: &State| state.phenotypes.clone());
    let dispatch = Dispatch::<State>::new();
    let set_value = Callback::from(move |value: Option<String>| {
        dispatch.reduce_mut(|state| state.phenotype = value)
    });

    html! {
        <DropdownForm {set_value} id={"phenotype"} text={"Wymierz fenotyp"} options={phenotypes} default={"Nieznany"} />
    }
}

#[function_component]
fn SelectGender() -> Html {
    let dispatch = Dispatch::<State>::new();
    let set_value = Callback::from(move |value: Option<String>| {
        dispatch.reduce_mut(|state| {
            state.gender = None;
            if let Some(gender) = value {
                if gender == "Samiec" {
                    state.gender = Some(Gender::Male);
                } else if gender == "Samica" {
                    state.gender = Some(Gender::Female);
                }
            }
        })
    });
    let options = Rc::new(vec!["Samiec".into(), "Samica".into()]);
    html! {
        <DropdownForm {set_value} id={"gender"} text={"Wymierz płeć"} {options} default={"Nieznana"}/>
    }
}

#[function_component]
pub fn AddAnimalTemp() -> Html {
    let dispatch = Dispatch::<State>::new();
    spawn_local(set_phenotypes(dispatch.clone()));
    spawn_local(set_litters(dispatch.clone()));
    let set_id = dispatch.reduce_mut_callback_with(|state, event| {
        state.id = event_to_text(event);
    });
    let set_status = dispatch.reduce_mut_callback_with(|state, event| {
        state.status = event_to_text(event);
    });
    let set_eye_color = dispatch.reduce_mut_callback_with(|state, event| {
        state.eye_color = event_to_text(event);
    });
    let set_hair = dispatch.reduce_mut_callback_with(|state, event| {
        state.hair = event_to_text(event);
    });

    let onclick = Callback::from({
        let navigator = use_navigator().unwrap();
        move |_| {
            let state = dispatch.get();
            let navigator = navigator.clone();
            if let (Some(id), Some(phenotype), Some(gender)) = (&state.id, &state.phenotype, &state.gender) {
                let animal = AnimalData {
                    id: id.clone(),
                    fenotyp: phenotype.clone(),
                    gender: gender.clone(),
                    litter: state.litter.clone(),
                    status: state.status.clone(),
                    mother: None,
                    father: None,
                    eye_color: state.eye_color.clone(),
                    hair: state.hair.clone(),
                };
                spawn_local(async move {
                    match backend_api::animal::post_animal(&animal).await {
                        Ok(()) => navigator.push(&Routes::GoToAnimal { id: animal.id }),
                        Err(_) => navigator.push(&Routes::ServerError)
                    }
                });
            }
            else {
                log(&format!("id: {:?}, phenotype: {:?}, gender: {:?}", state.id, state.phenotype, state.gender));
                navigator.push(&Routes::ServerError)
            }
        }
    });

    html! {
        <>
        <h1>{"Dodaj mysz"}</h1>
        <TextInput onchange={set_id} text={"id"} id={"id"}/>
        <SelectLitter />
        <SelectPhenotype />
        <SelectGender />
        <TextInput onchange={set_status} id="status" text="Status"/>
        <TextInput onchange={set_eye_color} id="eye_color" text="Kolor oka"/>
        <TextInput onchange={set_hair} id="hair" text="Włos"/>
        <button type="submit" {onclick} class="btn btn-primary mb-3">{"Dodaj Mysz"} </button>
        </>
    }
}
