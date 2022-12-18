pub mod animal;
pub mod litter;
pub mod phenotype;
mod base;

#[macro_use]
extern crate anyhow;

pub use animal::{get_all_animal, get_animal_by_id};
pub use phenotype::{get_genes, get_phenotypes};
pub use litter::{get_litter_list};