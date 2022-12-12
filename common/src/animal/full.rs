use serde::{Serialize, Deserialize};

use super::genes::AnimalGenes;
use super::photo::Photo;
use super::animal_data::{AnimalStatus, Gender, Litter};

#[derive(PartialEq, Serialize, Deserialize)]
pub struct Animal {
    pub id: String,
    pub fenotyp: String,
    pub gender: Gender,
    pub status: AnimalStatus,
    pub photos: Vec<Photo>,
    pub litter: Option<Litter>,
    pub genes: Option<AnimalGenes>
}