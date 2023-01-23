use crate::ConnectionDB;
use anyhow::{bail, Result};

pub async fn animal(id: &str, connection: &ConnectionDB) -> Result<()> {
    let result = sqlx::query!(
        "
        DELETE FROM ANIMAL
        WHERE id = $1",
        id
    )
    .execute(&connection.pool)
    .await?;
    if result.rows_affected() != 1 {
        bail!("Delete failed")
    }
    Ok(())
}
