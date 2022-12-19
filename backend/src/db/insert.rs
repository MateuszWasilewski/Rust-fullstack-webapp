

use anyhow::{Result, bail};
use sqlx::{Postgres, Pool};
use common::{litter::LitterData, animal::AnimalData};

pub async fn litter(litter: &LitterData, pool: &Pool<Postgres>) -> Result<()> {
    let result = sqlx::query!("
    INSERT INTO LITTER (id, mother, father)
    VALUES($1, $2, $3)", 
        &litter.id, 
        &litter.id_mother, 
        &litter.id_father)
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
    let result = sqlx::query!("
    INSERT INTO ANIMAL (id, phenotype, litter, gender_male)
    VALUES($1, $2, $3, true)",
        &animal.id,
        &animal.fenotyp,
        litter_id)
        .execute(pool)
        .await?;
    if result.rows_affected() != 1 {
        bail!("Insert failed")
    }
    Ok(())
}