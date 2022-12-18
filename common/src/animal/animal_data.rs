use serde::{Serialize, Deserialize};

#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub enum AnimalStatus {
    Alive,
    Dead,
    Unknown,
    Adopted
}

#[derive(Serialize, Deserialize, PartialEq)]
pub enum Gender {
    Male,
    Female
}


#[derive(PartialEq, Serialize, Deserialize)]
pub struct Litter {
    pub id: u32,
    pub mother: String,
    pub father: String,
}