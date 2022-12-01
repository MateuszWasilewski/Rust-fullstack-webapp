use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Phenotype {

    pub phenotype: String,
    pub variant: String,
    pub genes: HashMap<String, String>
}