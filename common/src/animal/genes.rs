use std::collections::HashMap;

#[derive(PartialEq)]
pub struct AnimalGenes {
    pub genes: HashMap<String, String>
}

impl AnimalGenes {
    pub fn get_genotype(&self, gene_names: &Vec<String>) -> String {
        let mut genotype = String::new();
        for gene in gene_names {
            let gene_str = match self.genes.get(gene) {
                Some(gene_value) => gene_value,
                None => "."
            };
            genotype.push_str(gene_str);
        }
        return genotype;
    }
}