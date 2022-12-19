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