pub mod photo;

use photo::Photo;

#[derive(Debug)]
enum AnimalStatus {
    Alive,
    Dead,
    Unknown
}

pub struct Animal {
    status: AnimalStatus,
    photos: Vec<photo::Photo>
}

impl Animal {
    pub fn new() -> Self {
        Animal {
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