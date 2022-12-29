use sqlx::{postgres::PgPoolOptions, Postgres, Pool};
use dotenv_codegen::dotenv;
use anyhow::Result;

pub mod select;
pub mod insert;

#[cfg(debug_assertions)]
static DB_URL: &str = dotenv!("DATABASE_URL");
#[cfg(not(debug_assertions))]
static DB_URL: &str = dotenv!("CLOUD_DATABASE_URL");


pub async fn connect_db() -> Result<Pool<Postgres>> {
    let pool = PgPoolOptions::new()
        .max_connections(8)
        .connect(DB_URL).await?;
    sqlx::migrate!("../db/migrations")
        .run(&pool)
        .await?;
    Ok(pool)
}

pub struct ConnectionDB {
    pub pool: Pool<Postgres>
}