use yew::{html, Html, function_component, use_state, Callback, platform::spawn_local};
use web_sys::MouseEvent;

use common::litter::LitterData;
use crate::common::input::{empty_str, TextInput};

#[function_component]
pub fn AddLitter() -> Html {
    let litter = use_state(empty_str);
    let mother = use_state(empty_str);
    let father = use_state(empty_str);
    let error_log = use_state(|| "");

    let onclick = Callback::from( {
        let litter = litter.clone();
        let mother = mother.clone();
        let father = father.clone();
        let error_log = error_log.clone();
        move |_: MouseEvent| {
            let litter = (*litter).clone();
            let mother = (*mother).clone();
            let father = (*father).clone();
            let error_log = error_log.clone();

            // TODO better error handling
            if litter.is_none() || mother.is_none() || father.is_none() {
                error_log.set("Some field is empty");
                return
            }

            let litter = LitterData {
                id: litter.unwrap(),
                id_father: father.unwrap(),
                id_mother: mother.unwrap()
            };
            spawn_local(async move {
                match backend_api::litter::post_litter(&litter).await {
                    Ok(_) => error_log.set(""),
                    Err(_) => error_log.set("Failed sending litter to backend")
                }
            });
    }});
    
    html! {
        <>
            <h1>{"Dodaj Miot"}</h1>
            <TextInput state={litter} id="id" text="Id miotu" />
            <TextInput state={mother} id="id_mother" text="Id matki" />
            <TextInput state={father} id="id_father" text="Id ojca" />
            <div class="col-auto">
                <button type="submit" {onclick} class="btn btn-primary mb-3">{"Dodaj miot"} </button>
            </div>
            {*error_log}
            </>
    }
}