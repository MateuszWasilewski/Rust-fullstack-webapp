use anyhow::Result;
use common::{litter::LitterData, AnimalData, AnimalFull, Phenotype, SearchResult};

use super::request_get;

pub async fn animal_by_id(id: &str) -> Result<AnimalFull> {
    request_get(&format!("/api/animal/{id}")).await
}

pub async fn all_animal() -> Result<Vec<AnimalData>> {
    request_get("/api/animal-list").await
}

pub async fn litter_list() -> Result<Vec<LitterData>> {
    request_get("/api/litter-list").await
}

pub async fn animal_litter_list(id: &str) -> Result<Vec<AnimalData>> {
    request_get(&format!("/api/animals-in-litter/{id}")).await
}

pub async fn genes() -> Result<Vec<String>> {
    request_get("/api/genes-list").await
}

pub async fn phenotypes() -> Result<Vec<Phenotype>> {
    request_get("/api/simple-phenotype-list").await
}

pub async fn phenotypes_full() -> Result<Vec<Phenotype>> {
    request_get("/api/phenotype-list").await
}

pub async fn search_results(query: &str) -> Result<Vec<SearchResult>> {
    request_get(&format!("/api/search/{query}")).await
}
