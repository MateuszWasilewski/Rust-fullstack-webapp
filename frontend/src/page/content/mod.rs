// use leptos::component;
// use yew::{function_component, html, Html};
// use yew_router::{prelude::Switch, BrowserRouter};
//
// pub mod add;
// mod animal;
// mod animal_list;
// mod error;
// mod home;
// mod litter_list;
// mod litter_page;
// mod not_found;
// mod phenotypes;
// mod search_results;
//
// use add::AddContent;
// use animal::Animal;
// use animal_list::AnimalList;
// use error::Error;
// use frontend_routing::Routes;
// use home::Home;
// use litter_list::LitterList;
// use litter_page::LitterPage;
// use not_found::NotFound;
// use phenotypes::Phenotypes;
// use search_results::SearchResults;

// #[function_component(Content)]
// pub fn get_content() -> Html {
//     html! {
//         <div class="container">
//         <BrowserRouter>
//             <Switch<Routes> render={switch} />
//         </BrowserRouter>
//         </div>
//     }
// }
//
// fn switch(route: Routes) -> Html {
//     match route {
//         Routes::Home => html! { <Home/> },
//         Routes::AnimalList => html! { <AnimalList /> },
//         Routes::GoToAnimal { id } => html! {
//             <Animal animal_id={id} />
//         },
//         Routes::GoToLitter { id } => html! {
//             <LitterPage {id} />
//         },
//         Routes::Phenotypes => html! { <Phenotypes /> },
//         Routes::Litters => html! { <LitterList /> },
//         Routes::Add => html! { <AddContent /> },
//         Routes::Search { query } => html! { <SearchResults {query} />},
//         Routes::NotFound => html! { <NotFound /> },
//         Routes::Error => html! { <Error /> },
//     }
// }
use super::paths;
use leptos::{component, view, IntoView, Scope};
use leptos_router::{Route, Routes};

mod home;
mod list;

use home::Home;
use list::animal::AnimalList;
use list::litter::LitterList;

#[component]
pub fn PageContent(cx: Scope) -> impl IntoView {
    view! {
        cx,
        <div class="container">
            <Routes>
                <Route path={paths::HOME} view=Home/>
                <Route path={paths::ANIMAL_LIST} view=AnimalList/>
                <Route path={paths::LITTER_LIST} view=LitterList/>
            </Routes>
        </div>
    }
}
