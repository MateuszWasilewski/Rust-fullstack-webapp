use sqlx::{postgres::PgPoolOptions};
use dotenv_codegen::dotenv;
use anyhow::Result;

pub mod select;
pub mod insert;
pub use sqlx::{Pool, Postgres};

#[cfg(debug_assertions)]
static DB_URL: &str = dotenv!("DATABASE_URL");
#[cfg(not(debug_assertions))]
static DB_URL: &str = dotenv!("CLOUD_DATABASE_URL");

pub async fn connect_db() -> Result<ConnectionDB> {
    let pool = PgPoolOptions::new()
        .max_connections(8)
        .connect(DB_URL).await?;
    sqlx::migrate!("../db/migrations")
        .run(&pool)
        .await?;
    Ok(ConnectionDB {
        pool: pool
    })
}

pub struct ConnectionDB {
    pool: Pool<Postgres>
}