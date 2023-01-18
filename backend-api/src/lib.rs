pub mod animal;
mod base;
pub mod litter;
pub mod phenotype;
mod search;

#[macro_use]
extern crate anyhow;

pub use animal::{get_all_animal, get_animal_by_id};
pub use litter::{get_litter_list, get_animal_litter_list};
pub use phenotype::{get_genes, get_phenotypes, get_phenotypes_full};
pub use search::{get_search_results};