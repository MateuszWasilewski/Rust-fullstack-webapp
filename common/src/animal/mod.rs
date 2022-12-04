pub mod photo;
pub mod genes;

mod compressed;
mod full;
mod animal_data;

pub use full::Animal;
pub use animal_data::{AnimalStatus, Gender, Litter};