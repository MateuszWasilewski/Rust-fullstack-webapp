
use common::{animal::Animal, Photo};

pub fn get_animal() -> Animal {
    let mut animal = Animal::new();
    let photo = common::animal::photo::Photo::new();
    animal.add_photo(photo);

    animal
}

pub fn get_all_animal() -> Vec<Animal> {
    let mut result = vec![
        Animal::new(),
        Animal::new(),
        Animal::new(),
    ];
    let photo = Photo::new();
    result[0].add_photo(photo);

    result
}