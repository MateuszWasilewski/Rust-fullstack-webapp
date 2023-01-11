use common::animal::Gender;
use common::AnimalData;
use yew::platform::spawn_local;
use yew::{function_component, html, Callback, Html};
use yewdux::dispatch::Dispatch;
use yewdux::prelude::use_selector;
use yewdux::store::Store;

use crate::common::input::event_to_text;
use crate::common::input::DropdownForm;
use crate::common::input::TextInput;
use anyhow::Result;
use std::rc::Rc;

async fn get_phenotypes() -> Result<Vec<String>> {
    Ok(backend_api::get_phenotypes()
        .await?
        .into_iter()
        .map(|phen| phen.phenotype)
        .collect())
}

async fn get_litters() -> Result<Vec<String>> {
    Ok(backend_api::get_litter_list()
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
}

async fn set_litters(dispatch: Dispatch<State>) {
    let litters = get_litters().await.unwrap_or_default();
    dispatch.reduce_mut(|state| state.litters = litters)
}

async fn set_phenotypes(dispatch: Dispatch<State>) {
    let phenotypes = get_phenotypes().await.unwrap_or_default();
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

    let onclick = Callback::from({
        let state = dispatch.get();
        move |_| {
            if state.id.is_none() || state.phenotype.is_none() || state.gender.is_none() {
                return;
            }
            let animal = AnimalData {
                id: state.id.clone().unwrap(),
                fenotyp: state.phenotype.clone().unwrap(),
                gender: state.gender.clone().unwrap(),
                litter: state.litter.clone(),
                status: state.status.clone(),
                mother: None,
                father: None,
                eye_color: None,    // TODO add option to set
                hair: None,         // TODO add option to set
            };
            spawn_local(async move {
                backend_api::animal::post_animal(&animal).await.unwrap();
            });
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
        <button type="submit" {onclick} class="btn btn-primary mb-3">{"Dodaj Mysz"} </button>
        </>
    }
}
