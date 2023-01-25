use super::request_delete;
use anyhow::Result;

pub async fn animal(id: &str) -> Result<()> {
    request_delete(&format!("/api/animal/{id}")).await
}

pub async fn litter(id: &str) -> Result<()> {
    request_delete(&format!("/api/litter/{id}")).await
}
