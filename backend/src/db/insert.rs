

use anyhow::{Result, bail};
use sqlx::{Postgres, Pool};
use common::litter::LitterData;

pub async fn litter(litter: &LitterData, pool: &Pool<Postgres>) -> Result<()> {
    let result = sqlx::query("
    INSERT INTO LITTER (id, mother, father)
    VALUES($1, $2, $3)")
        .bind(&litter.id)
        .bind(&litter.id_mother)
        .bind(&litter.id_father)
        .execute(pool)
        .await?;
    if result.rows_affected() != 1 {
        bail!("Insert failed")
    }
    Ok(())
}