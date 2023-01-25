
use anyhow::{Result, bail};

use common::{AnimalData, animal::Gender};

use crate::ConnectionDB;

pub async fn animal(animal: &AnimalData, connection: &ConnectionDB) -> Result<()> {
  let litter_id = animal.litter.clone();
  let result = sqlx::query!(
      "
      UPDATE ANIMAL SET 
        phenotype = $1,
        litter = $2,
        gender_male = $3,
        status = $4,
        eye_color = $5,
        hair = $6
      ",
      animal.fenotyp,
      litter_id,
      animal.gender == Gender::Male,
      animal.status,
      animal.eye_color,
      animal.hair
  )
  .execute(&connection.pool)
  .await?;
  if result.rows_affected() != 1 {
      bail!("Insert failed")
  }
  Ok(())
}