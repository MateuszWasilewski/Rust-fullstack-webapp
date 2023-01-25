use serde::{Deserialize, Serialize};

use super::genes::AnimalGenes;
use super::photo::Photo;

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub enum Gender {
    Male,
    Female,
}

#[derive(PartialEq, Serialize, Deserialize, Clone)]
pub struct AnimalFull {
    pub id: String,
    pub fenotyp: Option<String>,
    pub gender: Gender,
    pub status: Option<String>,
    pub eye_color: Option<String>,
    pub hair: Option<String>,
    pub photos: Vec<Photo>,
    pub litter: Option<String>,
    pub father: Option<String>,
    pub mother: Option<String>, 
    pub genes: Vec<AnimalGenes>
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct AnimalData {
    pub id: String,
    pub fenotyp: Option<String>,
    pub gender: Gender,
    pub status: Option<String>,
    pub eye_color: Option<String>,
    pub hair: Option<String>,
    pub litter: Option<String>,
    pub mother: Option<String>,
    pub father: Option<String>,
}

impl AnimalFull {
    pub fn add_photos(&mut self, mut photos: Vec<Photo>) {
        self.photos.append(&mut photos);
    }

    pub fn add_photo(&mut self, photo: Photo) {
        self.photos.push(photo);
    }
}

impl From<AnimalData> for AnimalFull {
    fn from(animal: AnimalData) -> Self {
        Self {
            id: animal.id,
            fenotyp: animal.fenotyp,
            genes: vec![],
            gender: animal.gender,
            status: animal.status,
            hair: animal.hair,
            eye_color: animal.eye_color,
            photos: vec![],
            litter: animal.litter,
            father: animal.father,
            mother: animal.mother
        }
    }
}