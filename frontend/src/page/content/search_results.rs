use common::SearchResult;
use web_sys::MouseEvent;
use yew::{function_component, Html, html, Properties, platform::spawn_local, Callback};
use yew_router::prelude::use_navigator;
use yewdux::{prelude::{Dispatch, use_selector}, store::Store};

use frontend_routing::Routes;

#[derive(PartialEq, Clone, Properties)]
pub struct SearchProps {
  pub query: String
}

#[derive(Store, Default, Clone, PartialEq)]
struct State {
   //query: Option<String>,
   data: Vec<SearchResult>
}

async fn fetch_data(query: String) {
  let dispatch = Dispatch::<State>::new();
  let response = backend_api::get_search_results(&query).await;
  if let Ok(data) = response {
    dispatch.reduce_mut(|state| state.data = data);
  }
}

#[function_component]
fn Results() -> Html {
  let results = use_selector(|state: &State| state.data.clone());
  let navigator = use_navigator().unwrap();

  let html_result = results.iter().map(|search_result| {
    let navigator = navigator.clone();
    let result = search_result.clone();
    let onclick = Callback::from(move |_: MouseEvent| {
        navigator.push(&result.route);
    });
    html! {
    <div class="row">
      <div class="col col-lg-2">
      <a class="nav-link active text-primary" href="javascript:void(0);" {onclick}>{&search_result.name}</a>
      </div>
      <div class="col">
        {&search_result.description}
      </div>
    </div>
  }}).collect::<Html>();

  html! {
    {html_result}
  }
}

#[function_component] 
pub fn SearchResults(props: &SearchProps) -> Html {
  //let dispatch = Dispatch::<State>::new();
  //dispatch.reduce_mut(|state| state.query = Some(props.query.clone()));
  spawn_local(fetch_data(props.query.clone()));

  html! {
    <Results />
  }
}