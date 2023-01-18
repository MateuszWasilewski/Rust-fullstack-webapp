pub mod animal;
pub mod litter;
pub mod phenotype;
mod search;

pub use animal::photo::Photo;
pub use animal::{AnimalData, AnimalFull};
pub use phenotype::{Phenotype, SimplePhenotype};
pub use search::SearchResult;