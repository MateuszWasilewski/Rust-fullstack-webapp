
use yew_router::prelude::Routable;

#[derive(Debug, Clone, PartialEq, Routable)]
pub enum Routes {
    #[at("/add/litter")]
    Litter,
    #[at("/add/animal")]
    Animal
}