use yew::{function_component, html, Html};
use yewdux::{prelude::Dispatch, store::Store};

use crate::common::input::{event_to_text, TextInput};

#[derive(PartialEq, Clone, Store, Default)]
struct State {
    name: Option<String>,
}

#[function_component]
pub fn AddPhenotype() -> Html {
    let dispatch = Dispatch::<State>::new();
    let onchange =
        dispatch.reduce_mut_callback_with(move |state, event| state.name = event_to_text(event));
    html! {
      <>
        <TextInput {onchange} id={"id"} text={"Nazwa"}/>
      </>
    }
}
