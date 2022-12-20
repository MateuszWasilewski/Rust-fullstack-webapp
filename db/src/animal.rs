

#[derive(sqlx::FromRow)]
pub struct AnimalDB {
    pub id: String,
    pub gender_male: bool,
    pub phenotype: String
}