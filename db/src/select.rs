use std::collections::HashMap;

use anyhow::{Result, anyhow};
use common::Phenotype;
use common::animal::Gender;
use sqlx::{Postgres, Pool};

use super::litter::LitterDB;
use super::animal::AnimalDB;
use common::{Animal, animal::AnimalStatus};
use common::litter::{LitterData, Litter};

pub async fn all_animal(pool: &Pool<Postgres>) -> Result<Vec<Animal>> {
    let rows = sqlx::query!("
        SELECT A.*, L.mother, L.father FROM ANIMAL A
        LEFT JOIN LITTER L
        ON A.litter = L.id").fetch_all(pool).await?;

    let animals = rows.into_iter().map(|row| {
        let litter = match row.litter.is_some() && row.mother.is_some() && row.father.is_some() {
            true => Some(Litter {
                id: row.litter.unwrap(),
                mother: row.mother.unwrap(),
                father: row.father.unwrap()
            }),
            false => None,
        };
        Animal {
            id: row.id,
            fenotyp: row.phenotype.unwrap(),
            gender: Gender::Male,
            status: AnimalStatus::Alive,
            litter: litter,
            photos: vec![]
        }
    }).collect();
    Ok(animals)
}

pub async fn animal(id: &str, pool: &Pool<Postgres>) -> Result<Animal> {
    //let rows = sqlx::query!("
    //    SELECT A.*, L.* FROM ANIMAL A
    //    LEFT JOIN LITTER L
    //    ON A.litter = L.id
    //    WHERE L.id = $1", &id).fetch_all(pool).await?;

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

pub async fn phenotype_list(pool: &Pool<Postgres>) -> Result<Vec<Phenotype>> {
    let rows = sqlx::query!(
        "SELECT id, phenotype_variant FROM PHENOTYPE")
        .fetch_all(pool)
        .await?;
    let phenotypes = rows.into_iter().map(|row| {
        Phenotype {
            phenotype: row.id,
            variant: row.phenotype_variant,
            genes: HashMap::new()
        }
    }).collect();

    Ok(phenotypes)
}