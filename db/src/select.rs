use std::collections::HashMap;

use anyhow::{anyhow, Result};
use common::animal::genes::AnimalGenes;
use common::animal::Gender;
use common::{Phenotype, Photo};
use sqlx::{Pool, Postgres};

use common::litter::LitterData;
use common::{AnimalData, AnimalFull};

struct DBAnimal {
    id: String,
    phenotype: String,
    gender_male: bool,
    status: Option<String>,
    eye_color: Option<String>,
    hair: Option<String>,
    litter: Option<String>,
    mother: Option<String>,
    father: Option<String>,
}

fn db_to_animal(row: DBAnimal) -> AnimalData {
    AnimalData {
        id: row.id,
        fenotyp: row.phenotype,
        gender: match row.gender_male {
            true => Gender::Male,
            false => Gender::Female,
        },
        status: row.status,
        litter: row.litter,
        father: row.father,
        mother: row.mother,
        eye_color: row.eye_color,
        hair: row.hair,
    }
}

pub async fn all_animal(pool: &Pool<Postgres>) -> Result<Vec<AnimalData>> {
    let animals = sqlx::query_as!(
        DBAnimal,
        "
        SELECT A.*, L.mother, L.father FROM ANIMAL A
        LEFT JOIN LITTER L
        ON A.litter = L.id"
    )
    .map(db_to_animal)
    .fetch_all(pool)
    .await?;

    Ok(animals)
}

pub async fn animals_in_litter(litter: &str, pool: &Pool<Postgres>) -> Result<Vec<AnimalData>> {
    let animals = sqlx::query_as!(
        DBAnimal,
        r#"
        SELECT 
            A.id,
            A.phenotype,
            A.gender_male,
            A.status,
            A.eye_color,
            A.hair,
            A.litter as "litter?",
            L.mother as "mother?",
            L.father as "father?"
        FROM ANIMAL A
        JOIN LITTER L
            ON A.litter = L.id
            WHERE A.litter = $1"#,
        litter
    )
    .map(db_to_animal)
    .fetch_all(pool)
    .await?;

    Ok(animals)
}

pub async fn animal(id: &str, pool: &Pool<Postgres>) -> Result<AnimalFull> {
    let row = sqlx::query!(
        r#"
        SELECT A.*, 
        L.mother as "mother?", 
        L.father as "father?"
        FROM ANIMAL A
        LEFT JOIN LITTER L
        ON A.litter = L.id
        WHERE A.id = $1"#,
        id
    )
    .fetch_optional(pool)
    .await?;
    match row {
        Some(animal) => Ok(AnimalFull {
            id: animal.id,
            fenotyp: animal.phenotype,
            gender: match animal.gender_male {
                true => common::animal::Gender::Male,
                false => common::animal::Gender::Female,
            },
            status: animal.status,
            photos: vec![],
            litter: animal.litter,
            father: animal.father,
            mother: animal.mother,
            eye_color: animal.eye_color,
            hair: animal.hair,
            genes: Vec::new(),
        }),
        None => Err(anyhow!("Animal is not present in db")),
    }
}

pub async fn litter_list(pool: &Pool<Postgres>) -> Result<Vec<LitterData>> {
    let litters = sqlx::query!("SELECT * FROM LITTER")
        .map(|row| LitterData {
            id: row.id,
            id_mother: row.mother,
            id_father: row.father,
        })
        .fetch_all(pool)
        .await?;

    Ok(litters)
}

pub async fn phenotype_list(pool: &Pool<Postgres>) -> Result<Vec<Phenotype>> {
    let phenotypes = sqlx::query!("SELECT * FROM PHENOTYPE")
        .map(|row| Phenotype {
            phenotype: row.name,
            variant: row.variant,
            genes: AnimalGenes::new(HashMap::new()),
        })
        .fetch_all(pool)
        .await?;

    Ok(phenotypes)
}

pub async fn phenotype_genes_list(pool: &Pool<Postgres>) -> Result<Vec<Phenotype>> {
    let phenotypes = sqlx::query!(
        r#"
        SELECT 
        P.name as "name!",
        P.variant as "variant!",
        G.genes as "genes?"
        FROM PHENOTYPE P
        LEFT JOIN GENOTYPE G 
        ON G.phenotype = P.name"#
    )
    .map(|row| {
        let mut genes: HashMap<String, String> = HashMap::new();
        let _ = row.genes.and_then(|value| {
            genes = serde_json::from_value(value).ok()?;
            Some(())
        });

        Phenotype {
            phenotype: row.name,
            variant: row.variant,
            genes: AnimalGenes::new(genes),
        }
    })
    .fetch_all(pool)
    .await?;
    Ok(phenotypes)
}

pub async fn photos_for_animal(id: &str, pool: &Pool<Postgres>) -> Result<Vec<Photo>> {
    let photos = sqlx::query!(
        "SELECT photo FROM ANIMAL_PHOTO
        WHERE animal = $1",
        id
    )
    .map(|row| Photo {
        path: row.photo,
        author: None,
    })
    .fetch_all(pool)
    .await?;

    Ok(photos)
}

pub async fn photos_for_litter(id: &str, pool: &Pool<Postgres>) -> Result<Vec<Photo>> {
    let photos = sqlx::query!(
        "
        SELECT photo FROM LITTER_PHOTO
        WHERE litter = $1",
        id
    )
    .map(|row| Photo {
        path: row.photo,
        author: None,
    })
    .fetch_all(pool)
    .await?;

    Ok(photos)
}

pub async fn genes_for_animal(id: &str, pool: &Pool<Postgres>) -> Result<Vec<AnimalGenes>> {
    let rows = sqlx::query!(
        r#"
        SELECT G.genes as "genes?"
            FROM ANIMAL A
        JOIN PHENOTYPE P
            ON A.phenotype = P.name
        LEFT JOIN GENOTYPE G
            ON P.name = G.phenotype
        WHERE A.id = $1"#,
        id
    )
    .map(|row| AnimalGenes {
        genes: match row
            .genes
            .and_then(|value| serde_json::from_value(value).ok())
        {
            Some(genes) => genes,
            None => HashMap::new(),
        },
    })
    .fetch_all(pool)
    .await?;

    Ok(rows)
}

pub async fn animals_for_query(query: &str, pool: &Pool<Postgres>) -> Result<Vec<AnimalData>> {
    let animals = sqlx::query_as!(
        DBAnimal,
        r#"
        SELECT A.*, 
        L.mother as "mother?", 
        L.father as "father?"
        FROM ANIMAL A
        LEFT JOIN LITTER L
        ON A.litter = L.id
        WHERE 
            regexp_count(A.id, $1, 1, 'i') > 0
        OR 
            regexp_count(A.phenotype, $1, 1, 'i') > 0
        "#,
        query
    )
    .map(db_to_animal)
    .fetch_all(pool)
    .await?;

    Ok(animals)
}
