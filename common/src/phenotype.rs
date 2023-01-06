use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Clone)]
pub struct Phenotype {
    pub phenotype: String,
    pub variant: String,
    pub genes: HashMap<String, String>,
}

#[derive(Serialize, Deserialize)]
pub struct SimplePhenotype {
    pub phenotype: String,
    pub variant: String,
}
