use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct Phenotype {
    pub phenotype: String,
    pub variant: String,
    pub genes: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Hash)]
pub struct SimplePhenotype {
    pub phenotype: String,
    pub variant: String,
}
