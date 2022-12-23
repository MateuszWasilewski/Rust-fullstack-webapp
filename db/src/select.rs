use std::collections::HashMap;

use anyhow::{Result, anyhow};
use common::{Phenotype, Photo};
use common::animal::Gender;
use sqlx::{Postgres, Pool};

use common::{AnimalFull, AnimalData};
use common::litter::LitterData;

pub async fn all_animal(pool: &Pool<Postgres>) -> Result<Vec<AnimalData>> {
    let rows = sqlx::query!("
        SELECT A.*, L.mother, L.father FROM ANIMAL A
        LEFT JOIN LITTER L
        ON A.litter = L.id")
        .fetch_all(pool)
        .await?;

    let animals = rows.into_iter().map(|row| {
        AnimalData {
            id: row.id,
            fenotyp: row.phenotype,
            gender: Gender::Male,
            status: row.status,
            litter: row.litter,
            father: row.father,
            mother: row.mother,
        }
    }).collect();
    Ok(animals)
}

pub async fn animal(id: &str, pool: &Pool<Postgres>) -> Result<AnimalFull> {
    let row = sqlx::query!(r#"
        SELECT A.*, 
        L.mother as "mother?", 
        L.father as "father?"
        FROM ANIMAL A
        LEFT JOIN LITTER L
        ON A.litter = L.id
        WHERE A.id = $1"#, id)
        .fetch_optional(pool)
        .await?;
    match row {
        Some(animal) => Ok ( AnimalFull { 
            id: animal.id.clone(),
            fenotyp: animal.phenotype.clone(),
            gender: match animal.gender_male {
                true => common::animal::Gender::Male,
                false => common::animal::Gender::Female
            },
            status: animal.status,
            photos: vec![],
            litter: animal.litter.clone(),
            father: animal.father.clone(),
            mother: animal.mother.clone(),
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

pub async fn photos_for_animal(id: &str, pool: &Pool<Postgres>) -> Result<Vec<Photo>> {
    let photos = sqlx::query!(
        "SELECT photo FROM ANIMAL_PHOTO
        WHERE animal = $1", id)   
        .map(|row| {
            Photo {
                path: row.photo,
                author: None
            }
        })
        .fetch_all(pool)
        .await?;

    Ok(photos)
}