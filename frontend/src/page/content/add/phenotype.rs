use yew::{function_component, Html, html};
use yewdux::{store::Store, prelude::Dispatch};

use crate::common::input::{TextInput, event_to_text};

#[derive(PartialEq, Clone, Store, Default)]
struct State {
  name: Option<String>
}

#[function_component]
pub fn AddPhenotype() -> Html {
  let dispatch = Dispatch::<State>::new();
  let onchange = dispatch.reduce_mut_callback_with(move |state, event|
    state.name = event_to_text(event)
  );
  html! {
    <>
      <TextInput {onchange} id={"id"} text={"Nazwa"}/>
    </>
  }
}