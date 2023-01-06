use common::litter::LitterData;
use yew::{html, Html, function_component, Callback, platform::spawn_local};
use yewdux::dispatch::Dispatch;
use yewdux::store::Store;

use crate::common::input::{event_to_text, TextInput};

#[derive(Default, PartialEq, Store, Clone)]
struct State {
    litter: Option<String>,
    mother: Option<String>,
    father: Option<String>,
}

#[function_component]
pub fn AddLitter() -> Html {
    let dispatch = Dispatch::<State>::new();
    let set_id = dispatch.reduce_mut_callback_with(|state, event| {
        state.litter = event_to_text(event);
    });
    let set_father = dispatch.reduce_mut_callback_with(|state, event| {
        state.father = event_to_text(event);
    });
    let set_mother = dispatch.reduce_mut_callback_with(|state, event| {
        state.mother = event_to_text(event);
    });
    
    let onclick = Callback::from( move |_| {
        let state = dispatch.get();
        let litter = state.litter.clone();
        let mother = state.mother.clone();
        let father = state.father.clone();

        // TODO better error handling
        if litter.is_none() || mother.is_none() || father.is_none() {
            return
        }

        let litter = LitterData {
            id: litter.unwrap(),
            id_father: father.unwrap(),
            id_mother: mother.unwrap()
        };
        spawn_local(async move {
            match backend_api::litter::post_litter(&litter).await {
                Ok(_) => (),
                Err(_) => ()
            }
        });
    });
    
    html! {
        <>
            <h1>{"Dodaj Miot"}</h1>
            <TextInput onchange={set_id} id="id" text="Id miotu" />
            <TextInput onchange={set_mother} id="id_mother" text="Id matki" />
            <TextInput onchange={set_father} id="id_father" text="Id ojca" />
            <div class="col-auto">
                <button type="submit" {onclick} class="btn btn-primary mb-3">{"Dodaj miot"} </button>
            </div>
            </>
    }
}