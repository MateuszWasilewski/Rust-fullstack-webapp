pub mod photo;

use photo::Photo;

#[derive(Debug, Clone, Copy)]
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
    pub miot: u32,
    pub mother: Option<String>,
    pub father: Option<String>,
}

impl Animal {
    pub fn add_photo(&mut self, photo: Photo) {
        self.photos.push(photo);
    }

    pub fn get_photo_count(&self) -> usize {
        self.photos.len()
    }
}