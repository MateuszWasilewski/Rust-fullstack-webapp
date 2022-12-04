use serde::{Serialize, Deserialize};
use chrono::NaiveDate;

#[derive(PartialEq, Debug)]
pub enum AnimalStatus {
    Alive,
    Dead,
    Unknown,
    Adopted
}

#[derive(Serialize, Deserialize)]
pub enum Gender {
    Male,
    Female
}


pub struct Litter {
    pub id: u32,
    pub mother: String,
    pub father: String,
    pub birth_date: NaiveDate
}