use sqlx::{Postgres, Pool};

pub struct ConnectionDB {
    pub pool: Pool<Postgres>
}