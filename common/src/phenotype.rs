use std::collections::HashMap;


pub struct Phenotype {
    pub phenotype: String,
    pub variant: String,
    pub genes: HashMap<String, String>
}