use web_sys::InputEvent;
use yew::{function_component, html, Html};
use yew_router::{prelude::use_navigator};
use yewdux::{prelude::Dispatch, store::Store};

use crate::common::input::input_event_to_text;
use crate::page::Routes;

#[derive(PartialEq, Store, Default, Clone)]
struct State {
    text: Option<String>,
}

#[function_component]
pub fn SearchBar() -> Html {
    let dispatch = Dispatch::<State>::new();
    let navigator = use_navigator().unwrap();
    let oninput = dispatch.reduce_mut_callback_with(|state, event: InputEvent| {
        state.text = input_event_to_text(event)
    });
    let onclick = dispatch.clone().reduce_callback(move |state| {
      if let Some(text) = dispatch.get().clone().text.clone() {
        navigator.push(&Routes::Search { query: text });
      }
      state
    });

    html! {
      <>
        <li>
          <input class="form-control" type="search" placeholder="Szukaj" {oninput} aria-label="Search" />
          </li>
        <li>
          <button class="btn btn-outline-success" type="submit" {onclick} >{"Szukaj"}</button>
        </li>
      </>
    }
}
