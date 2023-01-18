use yew_router::prelude::*;
use serde::{Serialize, Deserialize};

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
    Search { query: String }
}
