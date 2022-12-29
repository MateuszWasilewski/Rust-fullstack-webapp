use std::collections::HashMap;

use anyhow::{Result, anyhow};
use common::{Phenotype, Photo};
use common::animal::Gender;
use sqlx::{Postgres, Pool};

use common::{AnimalFull, AnimalData};
use common::litter::LitterData;

pub async fn all_animal(pool: &Pool<Postgres>) -> Result<Vec<AnimalData>> {
    let animals = sqlx::query!(r#"
        SELECT A.*, L.mother, L.father FROM ANIMAL A
        LEFT JOIN LITTER L
        ON A.litter = L.id"#)
        .map(|row| {
            AnimalData {
                id: row.id,
                fenotyp: row.phenotype,
                gender: match row.gender_male {
                    true => Gender::Male,
                    false => Gender::Female
                }, 
                status: row.status,
                litter: row.litter,
                father: row.father,
                mother: row.mother,
            }
        })
        .fetch_all(pool)
        .await?;

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
    let litters = sqlx::query!("SELECT * FROM LITTER")
        .map(|row| {
            LitterData {
                id: row.id,
                id_mother: row.mother,
                id_father: row.father
            }
        })
        .fetch_all(pool)
        .await?;

    Ok(litters)
}

pub async fn phenotype_list(pool: &Pool<Postgres>) -> Result<Vec<Phenotype>> {
    let phenotypes = sqlx::query!(
        "SELECT * FROM PHENOTYPE")
        .map(|row| {
            Phenotype {
                phenotype: row.name,
                variant: row.variant,
                genes: HashMap::new()
            }
        })
        .fetch_all(pool)
        .await?;

    Ok(phenotypes)
}

pub async fn phenotype_genes_list(pool: &Pool<Postgres>) -> Result<Vec<Phenotype>> {
    let phenotypes = sqlx::query!(r#"
        SELECT 
        P.name as "name!",
        P.variant as "variant!",
        G.genes as "genes?"
        FROM PHENOTYPE P
        LEFT JOIN GENOTYPE G 
        ON G.phenotype = P.name"#)
        .map(|row| {
            let mut genes: HashMap<String, String> = HashMap::new();
            let _ = row.genes.and_then(|value|  {
                genes = serde_json::from_value(value).ok()?;
                Some(())
            });

            Phenotype {
                phenotype: row.name,
                variant: row.variant,
                genes: genes
            }
        })
        .fetch_all(pool)
        .await?;
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