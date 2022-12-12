use common::{animal::{Animal, AnimalStatus, Litter, Gender}, Photo, Phenotype};
use chrono::NaiveDate;
use anyhow::Result;

#[macro_use]
extern crate anyhow;

//static BASE_URL: &str = "http://127.0.0.1:8000";
static BASE_URL: &str = "http://138.2.182.80:80";

pub async fn get_animal_by_id(id: &str) -> Result<Animal> {
    let animals = get_all_animal();
    for animal in animals {
        if animal.id == id {
            return Ok(animal)
        }
    }
    Err(anyhow!("No animal with given id"))
}

pub fn get_all_animal() -> Vec<Animal> {
    let animal_vec: Vec<Animal> = vec! [
        Animal {
            id: "28.M3".to_owned(),
            photos: vec![],
            gender: Gender::Male,
            status: AnimalStatus::Adopted,
            fenotyp: "dark splashed LH".to_owned(),
            litter: Some( Litter {
                id: 28,
                father: "R28.M".to_owned(),
                mother: "R28.F".to_owned(),
                birth_date: NaiveDate::parse_from_str("1/21/2022", "%m/%d/%Y").unwrap()
            }),
            genes: None
        },
        Animal {
            id: "24.F4".to_owned(),
            photos: vec![],
            gender: Gender::Female,
            status: AnimalStatus::Alive,
            fenotyp: "burmese LM".to_owned(),
            litter: Some(Litter {
                id: 24,
                mother: "R24.F".to_owned(),
                father: "R24.M".to_owned(),
                birth_date: NaiveDate::parse_from_str("3/11/2022", "%m/%d/%Y").unwrap()
            }),
            genes: None
        },
        Animal {
            id: "9.F4".to_owned(),
            photos: vec![],
            gender: Gender::Male,
            status: AnimalStatus::Adopted,
            fenotyp: "agouti".to_owned(),
            litter: Some(Litter {
                id: 9,
                mother: "R9.F38".to_owned(),
                father: "R9.M29".to_owned(),
                birth_date: NaiveDate::parse_from_str("3/8/2022", "%m/%d/%Y").unwrap()
            }),
            genes: None
        },
        Animal {
            id: "20.M4".to_owned(),
            photos: vec![],
            gender: Gender::Male,
            status: AnimalStatus::Adopted,
            fenotyp: "jasny beż".to_owned(),
            litter: Some(Litter {
                id: 20,
                mother: "R20.F52".to_owned(),
                father: "R20.M66".to_owned(),
                birth_date: NaiveDate::parse_from_str("3/11/2022", "%m/%d/%Y").unwrap()
            }),
            genes: None
        },
        Animal {
            id: "65.F5".to_owned(),
            photos: vec![
                Photo {
                    path: "65.F5.jpg".to_owned(),
                    author: Some("Edward".to_owned())
                },
            ],
            gender: Gender::Male,
            status: AnimalStatus::Alive,
            litter: Some(Litter { 
                id: 65,
                mother: "24.F4".to_owned(),
                father: "28.M3".to_owned(),
                birth_date: NaiveDate::parse_from_str("27-08-2022", "%d-%m-%Y").unwrap()
            }),
            fenotyp: "broken syjam".to_owned(),
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
            litter: Some(Litter { 
                id: 84, 
                mother: "31.F5".to_owned(), 
                father: "17.M2".to_owned(), 
                birth_date: NaiveDate::parse_from_str("01.09.2022", "%d.%m.%Y").unwrap() 
            }),
            fenotyp: "agouti".to_owned(),
            genes: None
        },
        Animal {
            id: "87.M2".to_owned(),
            fenotyp: "black".to_owned(),
            gender: Gender::Male,
            status: AnimalStatus::Alive,
            genes: None,
            photos: vec![],
            litter: Some(Litter { 
                id: 87, 
                mother: "22.F2".to_owned(), 
                father: "36.M1".to_owned(), 
                birth_date: NaiveDate::parse_from_str("05.09.2022", "%d.%m.%Y").unwrap() 
            })
        },
        Animal {
            id : "85.M1".to_owned(),
            gender: Gender::Male,
            fenotyp: "splashed".to_owned(),
            status: AnimalStatus::Alive,
            photos: vec![],
            litter: Some(Litter {
                id: 85,
                father: "20.M4".to_owned(),
                mother: "9.F4".to_owned(),
                birth_date: NaiveDate::parse_from_str("02.09.2022", "%d.%m.%Y").unwrap() 
            }),
            genes: None
        },
        Animal {
            id : "85.F2".to_owned(),
            gender: Gender::Female,
            fenotyp: "bew fuzzy".to_owned(),
            status: AnimalStatus::Alive,
            photos: vec![],
            litter: Some(Litter {
                id: 85,
                father: "20.M4".to_owned(),
                mother: "9.F4".to_owned(),
                birth_date: NaiveDate::parse_from_str("02.09.2022", "%d.%m.%Y").unwrap() 
            }),
            genes: None
        },
        Animal {
            id : "85.F3".to_owned(),
            gender: Gender::Female,
            fenotyp: "bew fuzzy".to_owned(),
            status: AnimalStatus::Alive,
            photos: vec![],
            litter: Some(Litter {
                id: 85,
                father: "20.M4".to_owned(),
                mother: "9.F4".to_owned(),
                birth_date: NaiveDate::parse_from_str("02.09.2022", "%d.%m.%Y").unwrap() 
            }),
            genes: None
        },
        Animal {
            id : "85.F4".to_owned(),
            gender: Gender::Female,
            fenotyp: "black fuzzy ".to_owned(),
            status: AnimalStatus::Alive,
            photos: vec![],
            litter: Some(Litter {
                id: 85,
                father: "20.M4".to_owned(),
                mother: "9.F4".to_owned(),
                birth_date: NaiveDate::parse_from_str("02.09.2022", "%d.%m.%Y").unwrap() 
            }),
            genes: None
        },
        Animal {
            id : "85.F5".to_owned(),
            gender: Gender::Female,
            fenotyp: "black fuzzy ".to_owned(),
            status: AnimalStatus::Alive,
            photos: vec![],
            litter: Some(Litter {
                id: 85,
                father: "20.M4".to_owned(),
                mother: "9.F4".to_owned(),
                birth_date: NaiveDate::parse_from_str("02.09.2022", "%d.%m.%Y").unwrap() 
            }),
            genes: None
        },
        Animal {
            id : "85.F6".to_owned(),
            gender: Gender::Female,
            fenotyp: "splashed fuzzy".to_owned(),
            status: AnimalStatus::Alive,
            photos: vec![],
            litter: Some(Litter {
                id: 85,
                father: "20.M4".to_owned(),
                mother: "9.F4".to_owned(),
                birth_date: NaiveDate::parse_from_str("02.09.2022", "%d.%m.%Y").unwrap() 
            }),
            genes: None
        },
        Animal {
            id : "85.F7".to_owned(),
            gender: Gender::Female,
            fenotyp: "black fuzzy łysa".to_owned(),
            status: AnimalStatus::Alive,
            photos: vec![],
            litter: Some(Litter {
                id: 85,
                father: "20.M4".to_owned(),
                mother: "9.F4".to_owned(),
                birth_date: NaiveDate::parse_from_str("02.09.2022", "%d.%m.%Y").unwrap() 
            }),
            genes: None
        },
        Animal {
            id : "85.F8".to_owned(),
            gender: Gender::Female,
            fenotyp: "black".to_owned(),
            status: AnimalStatus::Alive,
            photos: vec![],
            litter: Some(Litter {
                id: 85,
                father: "20.M4".to_owned(),
                mother: "9.F4".to_owned(),
                birth_date: NaiveDate::parse_from_str("02.09.2022", "%d.%m.%Y").unwrap() 
            }),
            genes: None
        },
        Animal {
            id : "85.F9".to_owned(),
            gender: Gender::Female,
            fenotyp: "black".to_owned(),
            status: AnimalStatus::Alive,
            photos: vec![],
            litter: Some(Litter {
                id: 85,
                father: "20.M4".to_owned(),
                mother: "9.F4".to_owned(),
                birth_date: NaiveDate::parse_from_str("02.09.2022", "%d.%m.%Y").unwrap() 
            }),
            genes: None
        },
    ];
    animal_vec
}

pub async fn get_genes() -> Result<Vec<String>> {
    let url = reqwest::Url::parse(BASE_URL)?;
    let url = url.join("/api/genes-list")?;

    let response = reqwest::get(url).await?;
    let parsed = response.json::<Vec<String>>().await?;

    Ok(parsed)
}

pub async fn get_phenotypes() -> Result<Vec<Phenotype>> {
    let url = reqwest::Url::parse(BASE_URL)?;
    let url = url.join("/api/phenotype-list")?;

    let response = reqwest::get(url).await?;
    let parsed = response.json::<Vec<Phenotype>>().await?;

    Ok(parsed)
}