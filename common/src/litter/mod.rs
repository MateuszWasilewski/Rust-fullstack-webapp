use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct LitterData {
    pub id: String,
    pub id_mother: String,
    pub id_father: String,
    // Date of birth
}

#[derive(PartialEq, Serialize, Deserialize)]
pub struct Litter {
    pub id: String,
    pub mother: String,
    pub father: String,
}
