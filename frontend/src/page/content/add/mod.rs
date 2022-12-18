use yew::{function_component, html, Html};
use yew_router::prelude::Switch;

pub mod routes;
mod litter;

use routes::Routes;

#[function_component(AddContent)]
pub fn get_add_content() -> Html {
    html! {
        <Switch<Routes> render={switch} />
    }
}

fn switch(route: Routes) -> Html {
    match route {
        Routes::Litter => html! { <h1>{"Add Aninal"}</h1>},
        Routes::Animal => html! {}
    }
}