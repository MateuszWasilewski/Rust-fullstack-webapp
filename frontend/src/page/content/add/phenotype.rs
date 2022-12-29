use yew::{function_component, Html, html, use_state};

use crate::common::input::TextInput;

#[function_component]
pub fn AddPhenotype() -> Html {
  let name = use_state(|| -> Option<String> {None});
  html! {
    <>
      <TextInput state={name.clone()} id={"id"} text={"Nazwa"}/>
      if let Some(text) = &*name {
        {text}
      }
    </>
  }
}