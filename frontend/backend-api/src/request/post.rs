use super::request_post;
use anyhow::Result;
use common::{litter::LitterData, AnimalData};

pub async fn animal(animal: &AnimalData) -> Result<()> {
    request_post(&format!("/api/animal/{}", animal.id), animal).await
}

pub async fn litter(litter: &LitterData) -> Result<()> {
    request_post("/api/litter", litter).await
}
