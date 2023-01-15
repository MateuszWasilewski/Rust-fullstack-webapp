use anyhow::{bail, Result};
use common::animal::Gender;
use common::{animal::AnimalData, litter::LitterData};
use common::{Phenotype, Photo, SimplePhenotype};
use sqlx::{Pool, Postgres};

pub async fn litter(litter: &LitterData, pool: &Pool<Postgres>) -> Result<()> {
    let result = sqlx::query!(
        "
        INSERT INTO LITTER (id, mother, father)
        VALUES($1, $2, $3)",
        &litter.id,
        &litter.id_mother,
        &litter.id_father
    )
    .execute(pool)
    .await?;
    if result.rows_affected() != 1 {
        bail!("Insert failed")
    }
    Ok(())
}

pub async fn animal(animal: &AnimalData, pool: &Pool<Postgres>) -> Result<()> {
    let litter_id = animal.litter.clone();
    // TODO
    let result = sqlx::query!(
        "
        INSERT INTO ANIMAL (id, phenotype, litter, gender_male, status, eye_color, hair)
        VALUES($1, $2, $3, $4, $5, $6, $7)",
        &animal.id,
        &animal.fenotyp,
        litter_id,
        animal.gender == Gender::Male,
        animal.status,
        animal.eye_color,
        animal.hair
    )
    .execute(pool)
    .await?;
    if result.rows_affected() != 1 {
        bail!("Insert failed")
    }
    Ok(())
}

pub async fn phenotype(phenotype: &SimplePhenotype, pool: &Pool<Postgres>) -> Result<()> {
    let result = sqlx::query!(
        "
        INSERT INTO PHENOTYPE (name, variant)
        VALUES ($1, $2)",
        &phenotype.phenotype,
        &phenotype.variant
    )
    .execute(pool)
    .await?;
    if result.rows_affected() != 1 {
        bail!("Insert failed")
    }
    Ok(())
}

pub async fn genes(phenotype: &Phenotype, pool: &Pool<Postgres>) -> Result<()> {
    let genes = serde_json::to_value(&phenotype.genes)?;
    let result = sqlx::query!(
        "
        INSERT INTO GENOTYPE (phenotype, genes)
        VALUES ($1, $2)",
        &phenotype.phenotype,
        genes
    )
    .execute(pool)
    .await?;
    if result.rows_affected() != 1 {
        bail!("Insert failed")
    }
    Ok(())
}

pub async fn photo(photo: &Photo, pool: &Pool<Postgres>) -> Result<()> {
    let result = sqlx::query!(
        "
        INSERT INTO PHOTO (path)
        VALUES ($1)",
        photo.path
    )
    .execute(pool)
    .await?;

    if result.rows_affected() != 1 {
        bail!("Insert failed")
    }
    Ok(())
}

pub async fn link_litter_to_photo(litter: &str, photo: &str, pool: &Pool<Postgres>) -> Result<()> {
    let result = sqlx::query!(
        "
       INSERT INTO LITTER_PHOTO (litter, photo)
       VALUES ($1, $2)",
        litter,
        photo
    )
    .execute(pool)
    .await?;

    if result.rows_affected() != 1 {
        bail!("Insert failed")
    }
    Ok(())
}
