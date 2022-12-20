use serde::{Serialize, Deserialize};

//use super::genes::AnimalGenes;
use super::photo::Photo;
use super::animal_data::{AnimalStatus, Gender};

#[derive(PartialEq, Serialize, Deserialize)]
pub struct Animal {
    pub id: String,
    pub fenotyp: String,
    pub gender: Gender,
    pub status: AnimalStatus,
    pub photos: Vec<Photo>,
    pub litter: Option<String>,
    pub father: Option<String>,
    pub mother: Option<String>
    //pub genes: Option<AnimalGenes>
}

#[derive(Serialize, Deserialize)]
pub struct AnimalData {
    pub id: String,
    pub fenotyp: String,
    pub gender: Gender,
    pub status: AnimalStatus,
    pub litter: Option<String>
}