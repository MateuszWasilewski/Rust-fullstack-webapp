use anyhow::{Result, anyhow};
use sqlx::{Postgres, Pool};

use super::litter::LitterDB;
use super::animal::AnimalDB;
use common::{Animal, animal::AnimalStatus};
use common::litter::LitterData;

pub async fn all_animal(pool: &Pool<Postgres>) -> Result<Vec<Animal>> {
    let rows = sqlx::query_as::<_, AnimalDB>
        ("SELECT * FROM ANIMAL")
        .fetch_all(pool).await?;

    let animals = rows.into_iter().map(|animal| {
        Animal {
            id: animal.id,
            fenotyp: animal.phenotype,
            gender: match animal.gender_male {
                true => common::animal::Gender::Male,
                false => common::animal::Gender::Female
            },
            status: AnimalStatus::Unknown,
            photos: vec![],
            litter: None,
            genes: None
        }
    }).collect();
    Ok(animals)
}

pub async fn animal(id: &str, pool: &Pool<Postgres>) -> Result<Animal> {
    let rows = sqlx::query_as::<_, AnimalDB>
        ("SELECT * FROM ANIMAL WHERE id = $1")
        .bind(id)
        .fetch_all(pool).await?;
   
    let animal = rows.get(0);
    match animal {
        Some(animal) => Ok ( Animal { 
            id: animal.id.clone(),
            fenotyp: animal.phenotype.clone(),
            gender: match animal.gender_male {
                true => common::animal::Gender::Male,
                false => common::animal::Gender::Female
            },
            status: AnimalStatus::Unknown,
            photos: vec![],
            litter: None,
            genes: None
        }),
        None => Err(anyhow!("Animal is not present in db"))
    }
}

pub async fn litter_list(pool: &Pool<Postgres>) -> Result<Vec<LitterData>> {
    let rows = sqlx::query_as::<_, LitterDB>
        ("SELECT * FROM LITTER")
        .fetch_all(pool).await?;

    let litters: Vec<LitterData> = rows.into_iter().map(|litter| {
        LitterData {
            id: litter.id,
            id_mother: litter.mother,
            id_father: litter.father
        }
    }).collect();

    Ok(litters)
}