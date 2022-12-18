use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct LitterData {
    pub id: String,
    pub id_mother: String,
    pub id_father: String,
    // Date of birth
}