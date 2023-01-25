use yew_router::prelude::*;
use serde::{Serialize, Deserialize};

mod add;

pub use add::Routes as RoutesAdd;

#[derive(Debug, Clone, PartialEq, Routable, Serialize, Deserialize)]
pub enum Routes {
    #[at("/")]
    Home,
    #[at("/animal-list")]
    AnimalList,
    #[at("/animal/:id")]
    GoToAnimal { id: String },
    #[at("/litter/:id")]
    GoToLitter { id: String },
    #[at("/phenotypes")]
    Phenotypes,
    #[at("/litter-list")]
    Litters,
    #[at("/add/*")]
    Add,
    #[at("/search/:query")]
    Search { query: String },
    #[at("/error")]
    ServerError,
    #[not_found]
    #[at("/404")]
    NotFound,
}
