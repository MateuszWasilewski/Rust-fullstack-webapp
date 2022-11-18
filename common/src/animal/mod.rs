pub mod photo;
use chrono::NaiveDate;

#[derive(PartialEq, Debug)]
pub enum AnimalStatus {
    Alive,
    Dead,
    Unknown,
    Adopted
}

pub struct Animal {
    pub id: String,
    pub fenotyp: String,
    pub status: AnimalStatus,
    pub photos: Vec<photo::Photo>,
    pub litter: Option<Litter>,
}

pub struct Litter {
    pub id: u32,
    pub mother: String,
    pub father: String,
    pub birth_date: NaiveDate
}