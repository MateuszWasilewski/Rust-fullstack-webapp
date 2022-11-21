use common::{animal::{Animal, AnimalStatus, Litter, Gender}, Photo, Phenotype};
use chrono::NaiveDate;

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
            gender: Gender::Male,
            status: AnimalStatus::Alive,
            litter: Some(Litter { 
                id: 65,
                mother: "24.F4".to_owned(),
                father: "28.M3".to_owned(),
                birth_date: NaiveDate::parse_from_str("27-08-2022", "%d-%m-%Y").unwrap()
            }),
            fenotyp: "broken tricolor LH czerwone oczy".to_owned(),
            genes: None
        },
        Animal {
            id: "66.F3".to_owned(),
            photos: vec![],
            status: AnimalStatus::Adopted,
            gender: Gender::Female,
            litter: Some(Litter { 
                id: 66,
                mother: "30.F4".to_owned(),
                father: "4.M2".to_owned(),
                birth_date: NaiveDate::parse_from_str("27-08-2022", "%d-%m-%Y").unwrap()
            }),
            fenotyp: "pearl".to_owned(),
            genes: None
        },
        Animal {
            id: "84.M1".to_owned(),
            gender: Gender::Male,
            photos: vec![],
            status: AnimalStatus::Alive,
            litter: None,
            fenotyp: "agouti".to_owned(),
            genes: None
        }
    ];
    animal_vec
}

pub fn get_genes() -> Vec<String> {
    vec! [
        "A".to_owned(),
        "ASIP2".to_owned(),
        "TYRP1".to_owned(),
        "TYRP2"	.to_owned(),
        "TYR".to_owned(),
        "TYR2".to_owned(),
        "MYO5A".to_owned(),
        "MYO5A2".to_owned(),
        "OCA2".to_owned(),
        "OCA3".to_owned(),
    ]
}

pub fn get_phenotypes() -> Vec<Phenotype> {
    vec! [
        Phenotype {
            phenotype: "agouti".to_owned(),
            variant: "ticked".to_owned(),
            genes: [
                ("A".to_owned(), "A".to_owned()),
                ("TYRP1".to_owned(), "B".to_owned()),
                ("TYR".to_owned(), "C".to_owned()),
                ("MYO5A".to_owned(), "D".to_owned()),
                ("OCA2".to_owned(), "P".to_owned())
            ].into_iter().collect()
        }
    ]
}