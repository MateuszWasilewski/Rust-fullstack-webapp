use yew::{function_component, Html, html};
use yew_router::prelude::{Switch};

mod animal_list;

use animal_list::AnimalList;
use super::routes::Routes;

#[function_component(Content)]
pub fn get_content() -> Html {
    html! {
        <div class="container">
            <Switch<Routes> render={Switch::render(switch)} />
        </div>
    }
}

fn switch(route: &Routes) -> Html {
    match route {
        Routes::Home => html! { <h1>{"Home"} </h1> },
        Routes::List => html! { <AnimalList /> },
        Routes::GoToAnimal { id: _ } => html!{ <h1> {"Animal page"} </h1> }
    } 
}