use serde::{Deserialize, Serialize};

use crate::animal::genes::AnimalGenes;

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct Phenotype {
    pub phenotype: String,
    pub variant: String,
    pub genes: AnimalGenes
}

#[derive(Serialize, Deserialize, Hash)]
pub struct SimplePhenotype {
    pub phenotype: String,
    pub variant: String,
}