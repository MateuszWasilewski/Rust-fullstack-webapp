use serde::{Deserialize, Serialize};

//use super::genes::AnimalGenes;
use super::photo::Photo;

#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub enum Gender {
    Male,
    Female,
}

#[derive(PartialEq, Serialize, Deserialize)]
pub struct AnimalFull {
    pub id: String,
    pub fenotyp: String,
    pub gender: Gender,
    pub status: Option<String>,
    pub photos: Vec<Photo>,
    pub litter: Option<String>,
    pub father: Option<String>,
    pub mother: Option<String>, //pub genes: Option<AnimalGenes>
}

#[derive(Serialize, Deserialize)]
pub struct AnimalData {
    pub id: String,
    pub fenotyp: String,
    pub gender: Gender,
    pub status: Option<String>,
    pub litter: Option<String>,
    pub mother: Option<String>,
    pub father: Option<String>,
}

impl AnimalFull {
    pub fn set_photos(&mut self, photos: Vec<Photo>) -> () {
        self.photos = photos;
    }
}
