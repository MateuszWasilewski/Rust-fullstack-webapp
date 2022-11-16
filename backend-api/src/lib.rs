
use common::animal::Animal;

pub fn get_animal() -> Animal {
    let mut animal = Animal::new();
    let photo = common::animal::photo::Photo::new();
    animal.add_photo(photo);

    animal
}