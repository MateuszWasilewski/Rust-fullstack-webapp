use chrono::NaiveDate;

pub mod photo;
pub mod genes;

use genes::AnimalGenes;

#[derive(PartialEq, Debug)]
pub enum AnimalStatus {
    Alive,
    Dead,
    Unknown,
    Adopted
}

pub enum Gender {
    Male,
    Female
}

pub struct Animal {
    pub id: String,
    pub fenotyp: String,
    pub gender: Gender,
    pub status: AnimalStatus,
    pub photos: Vec<photo::Photo>,
    pub litter: Option<Litter>,
    pub genes: Option<AnimalGenes>
}

pub struct Litter {
    pub id: u32,
    pub mother: String,
    pub father: String,
    pub birth_date: NaiveDate
}