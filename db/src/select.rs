use std::collections::HashMap;

use anyhow::{Result, anyhow};
use common::Phenotype;
use common::animal::Gender;
use sqlx::{Postgres, Pool};

use common::{Animal, animal::AnimalStatus};
use common::litter::LitterData;

pub async fn all_animal(pool: &Pool<Postgres>) -> Result<Vec<Animal>> {
    let rows = sqlx::query!("
        SELECT A.*, L.mother, L.father FROM ANIMAL A
        LEFT JOIN LITTER L
        ON A.litter = L.id")
        .fetch_all(pool)
        .await?;

    let animals = rows.into_iter().map(|row| {
        Animal {
            id: row.id,
            fenotyp: row.phenotype,
            gender: Gender::Male,
            status: AnimalStatus::Alive,
            litter: row.litter,
            father: row.father,
            mother: row.mother,
            photos: vec![]
        }
    }).collect();
    Ok(animals)
}

pub async fn animal(id: &str, pool: &Pool<Postgres>) -> Result<Animal> {
    let rows = sqlx::query!("
        SELECT A.*, L.mother, L.father FROM ANIMAL A
        LEFT JOIN LITTER L
        ON A.litter = L.id
        WHERE L.id = $1", &id)
        .fetch_all(pool)
        .await?;
    
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
            litter: animal.litter.clone(),
            father: Some(animal.father.clone()),
            mother: Some(animal.mother.clone()),
        }),
        None => Err(anyhow!("Animal is not present in db"))
    } 
}

pub async fn litter_list(pool: &Pool<Postgres>) -> Result<Vec<LitterData>> {
    let rows = sqlx::query!("SELECT * FROM LITTER")
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

pub async fn phenotype_list(pool: &Pool<Postgres>) -> Result<Vec<Phenotype>> {
    let rows = sqlx::query!(
        "SELECT name, variant FROM PHENOTYPE")
        .fetch_all(pool)
        .await?;
    let phenotypes = rows.into_iter().map(|row| {
        Phenotype {
            phenotype: row.name,
            variant: row.variant,
            genes: HashMap::new()
        }
    }).collect();

    Ok(phenotypes)
}