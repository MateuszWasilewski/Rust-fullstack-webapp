use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(PartialEq, Serialize, Deserialize, Clone, Default)]
pub struct AnimalGenes {
    pub genes: HashMap<String, String>,
}

impl AnimalGenes {
    pub fn get_genotype(&self, gene_names: &Vec<String>) -> String {
        let mut genotype = String::new();
        for gene in gene_names {
            let gene_str = match self.genes.get(gene) {
                Some(gene_value) => gene_value,
                None => ".",
            };
            genotype.push_str(gene_str);
        }
        return genotype;
    }

    pub fn new(genes: HashMap<String, String>) -> Self {
        AnimalGenes {
            genes
        }
    }
}
