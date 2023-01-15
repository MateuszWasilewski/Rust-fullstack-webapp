use serde::{Deserialize, Serialize};

//use super::genes::AnimalGenes;
use super::photo::Photo;

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
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
    pub eye_color: Option<String>,
    pub hair: Option<String>,
    pub photos: Vec<Photo>,
    pub litter: Option<String>,
    pub father: Option<String>,
    pub mother: Option<String>, //pub genes: Option<AnimalGenes>
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct AnimalData {
    pub id: String,
    pub fenotyp: String,
    pub gender: Gender,
    pub status: Option<String>,
    pub eye_color: Option<String>,
    pub hair: Option<String>,
    pub litter: Option<String>,
    pub mother: Option<String>,
    pub father: Option<String>,
}

impl AnimalFull {
    pub fn add_photos(&mut self, mut photos: Vec<Photo>) -> () {
        self.photos.append(&mut photos);
    }
}
