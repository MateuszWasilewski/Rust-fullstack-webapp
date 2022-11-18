use common::{animal::Animal, animal::AnimalStatus};


pub fn get_animal_by_id(id: String) -> Option<Animal> {
    let animals = get_all_animal();
    for animal in animals {
        if animal.id == id {
            return Some(animal)
        }
    }
    None
}

pub fn get_all_animal() -> Vec<Animal> {
    let animal_vec: Vec<Animal> = vec! [
        Animal {
            id: "65.M12".to_owned(),
            photos: vec![],
            status: AnimalStatus::Alive,
            miot: 65,
            mother: Some("24.F4".to_owned()),
            father: Some("28.M3".to_owned()),
            fenotyp: "broken tricolor LH czerwone oczy".to_owned()
        },
        Animal {
            id: "66.F3".to_owned(),
            photos: vec![],
            status: AnimalStatus::Adopted,
            miot: 66,
            mother: Some("30.F4".to_owned()),
            father: Some("4.M2".to_owned()),
            fenotyp: "pearl".to_owned()
        },
        Animal {
            id: "84.M1".to_owned(),
            photos: vec![],
            status: AnimalStatus::Alive,
            miot: 84,
            mother: None,
            father: None,
            fenotyp: "agouti".to_owned()
        }
    ];
    animal_vec
}