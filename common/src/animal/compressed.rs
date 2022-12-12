use super::animal_data::{AnimalStatus, Gender, Litter};

#[derive(PartialEq)]
pub struct Animal {
    pub id: String,
    pub fenotyp: String,
    pub gender: Gender,
    pub status: AnimalStatus,
    pub litter: Option<Litter>,
}