use common::{animal::{Animal, AnimalStatus, Litter}, Photo};


pub fn get_animal_by_id(id: &str) -> Option<Animal> {
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
            photos: vec![
                Photo {
                    path: "mouse_img.JPG".to_owned(),
                    author: Some("Edward".to_owned())
                },
                Photo {
                    path: "46M4.JPG".to_owned(),
                    author: None
                }
            ],
            status: AnimalStatus::Alive,
            litter: Some(Litter { 
                id: 65,
                mother: "24.F4".to_owned(),
                father: "28.M3".to_owned()
            }),
            fenotyp: "broken tricolor LH czerwone oczy".to_owned()
        },
        Animal {
            id: "66.F3".to_owned(),
            photos: vec![],
            status: AnimalStatus::Adopted,
            litter: Some(Litter { 
                id: 66,
                mother: "30.F4".to_owned(),
                father: "4.M2".to_owned()
            }),
            fenotyp: "pearl".to_owned()
        },
        Animal {
            id: "84.M1".to_owned(),
            photos: vec![],
            status: AnimalStatus::Alive,
            litter: None,
            fenotyp: "agouti".to_owned()
        }
    ];
    animal_vec
}