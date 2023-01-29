use std::collections::HashMap;

use anyhow::{anyhow, Result};
use common::animal::genes::AnimalGenes;
use common::animal::Gender;
use common::{Phenotype, Photo};

use crate::ConnectionDB;
use common::litter::LitterData;
use common::AnimalData;
use types::AncestryNode;

struct DBAnimal {
    id: String,
    phenotype: Option<String>,
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

pub async fn all_animal(connection: &ConnectionDB) -> Result<Vec<AnimalData>> {
    let animals = sqlx::query_as!(
        DBAnimal,
        r#"
        SELECT 
            A.*,
            L.mother as "mother?",
            L.father as "father?"
        FROM ANIMAL A
        LEFT JOIN LITTER L
            ON A.litter = L.id
            ORDER BY A.id ASC"#
    )
    .map(db_to_animal)
    .fetch_all(&connection.pool)
    .await?;

    Ok(animals)
}

pub async fn animals_in_litter(litter: &str, connection: &ConnectionDB) -> Result<Vec<AnimalData>> {
    let animals = sqlx::query_as!(
        DBAnimal,
        r#"
        SELECT 
            A.*,
            L.mother as "mother?",
            L.father as "father?"
        FROM ANIMAL A
        JOIN LITTER L
            ON A.litter = L.id
            WHERE A.litter = $1"#,
        litter
    )
    .map(db_to_animal)
    .fetch_all(&connection.pool)
    .await?;

    Ok(animals)
}

pub async fn animal(id: &str, connection: &ConnectionDB) -> Result<AnimalData> {
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
    .fetch_optional(&connection.pool)
    .await?;
    match row {
        Some(animal) => Ok(AnimalData {
            id: animal.id,
            fenotyp: animal.phenotype,
            gender: match animal.gender_male {
                true => common::animal::Gender::Male,
                false => common::animal::Gender::Female,
            },
            status: animal.status,
            litter: animal.litter,
            father: animal.father,
            mother: animal.mother,
            eye_color: animal.eye_color,
            hair: animal.hair,
        }),
        None => Err(anyhow!("Animal is not present in db")),
    }
}

pub async fn litter_list(connection: &ConnectionDB) -> Result<Vec<LitterData>> {
    let litters = sqlx::query!(
        "SELECT id, mother, father 
        FROM LITTER
        ORDER BY id"
    )
    .map(|row| LitterData {
        id: row.id,
        id_mother: row.mother,
        id_father: row.father,
    })
    .fetch_all(&connection.pool)
    .await?;

    Ok(litters)
}

pub async fn phenotype_list(connection: &ConnectionDB) -> Result<Vec<Phenotype>> {
    let phenotypes = sqlx::query!(
        r#"
        SELECT 
        name,
        variant
        FROM PHENOTYPE
        ORDER BY name
        "#
    )
    .map(|row| Phenotype {
        phenotype: row.name,
        variant: row.variant,
        genes: AnimalGenes::new(HashMap::new()),
    })
    .fetch_all(&connection.pool)
    .await?;

    Ok(phenotypes)
}

pub async fn phenotype_genes_list(connection: &ConnectionDB) -> Result<Vec<Phenotype>> {
    let phenotypes = sqlx::query!(
        r#"
        SELECT 
        P.name as "name!",
        P.variant as "variant!",
        G.genes as "genes?"
        FROM PHENOTYPE P
        LEFT JOIN GENOTYPE G 
        ON G.phenotype = P.name
        ORDER BY P.variant, P.name"#
    )
    .map(|row| {
        let mut genes: AnimalGenes = AnimalGenes::new(HashMap::new());
        row.genes.map(|value| {
            genes = serde_json::from_value(value).ok()?;
            Some(())
        });

        Phenotype {
            phenotype: row.name,
            variant: row.variant,
            genes: genes,
        }
    })
    .fetch_all(&connection.pool)
    .await?;
    Ok(phenotypes)
}

pub async fn photos_for_animal(id: &str, connection: &ConnectionDB) -> Result<Vec<Photo>> {
    let photos = sqlx::query!(
        "SELECT photo FROM ANIMAL_PHOTO
        WHERE animal = $1",
        id
    )
    .map(|row| Photo {
        path: row.photo,
        author: None,
    })
    .fetch_all(&connection.pool)
    .await?;

    Ok(photos)
}

pub async fn photos_for_litter(id: &str, connection: &ConnectionDB) -> Result<Vec<Photo>> {
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
    .fetch_all(&connection.pool)
    .await?;

    Ok(photos)
}

pub async fn genes_for_animal(id: &str, connection: &ConnectionDB) -> Result<Vec<AnimalGenes>> {
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
    .map(|row| {
        match row
            .genes
            .and_then(|value| serde_json::from_value(value).ok())
        {
            Some(genes) => genes,
            None => AnimalGenes::default(),
        }
    })
    .fetch_all(&connection.pool)
    .await?;

    Ok(rows)
}

pub async fn animals_for_query(query: &str, connection: &ConnectionDB) -> Result<Vec<AnimalData>> {
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
        ORDER BY A.id ASC
        "#,
        query
    )
    .map(db_to_animal)
    .fetch_all(&connection.pool)
    .await?;

    Ok(animals)
}

pub async fn ancestry(
    id: &str,
    max_depth: i32,
    connection: &ConnectionDB,
) -> Result<Vec<AncestryNode>> {
    let ancestries = sqlx::query_as!(
        AncestryNode,
        r#"
        WITH RECURSIVE ancestry(id, mother, father, depth) AS (
            SELECT A.id, L.mother, L.father, 0 as depth from ANIMAL A
                LEFT JOIN LITTER L ON A.litter = L.id
                WHERE A.id = $1
            UNION ALL
            SELECT 
                A.id, L.mother, L.father, AN.depth + 1
            FROM ancestry AN
            JOIN ANIMAL A ON 
                AN.mother = A.id OR AN.father = A.id
            LEFT JOIN LITTER L ON A.litter = L.id
            WHERE AN.depth + 1 < $2
        )
        SELECT 
            id as "id!",
            mother as "mother?",
            father as "father?",
            depth as "depth!"
        FROM ancestry
    "#,
        id,
        max_depth
    )
    .fetch_all(&connection.pool)
    .await?;

    Ok(ancestries)
}
