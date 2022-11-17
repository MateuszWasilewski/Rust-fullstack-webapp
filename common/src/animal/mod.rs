pub mod photo;

use photo::Photo;

#[derive(Debug)]
pub enum AnimalStatus {
    Alive,
    Dead,
    Unknown
}

pub struct Animal {
    pub id: String,
    status: AnimalStatus,
    photos: Vec<photo::Photo>
}

impl Animal {
    pub fn new() -> Self {
        Animal {
            id: "unknown".to_owned(),
            status: AnimalStatus::Unknown,
            photos: vec![]
        }
    }

    pub fn add_photo(&mut self, photo: Photo) {
        self.photos.push(photo);
    }

    pub fn get_photo_count(&self) -> usize {
        self.photos.len()
    }
}