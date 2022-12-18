

#[derive(sqlx::FromRow)]
pub struct LitterDB {
    pub id: String,
    pub mother: String,
    pub father: String,
}