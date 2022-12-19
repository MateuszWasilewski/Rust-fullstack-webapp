use crate::base::get_url;
use anyhow::Result;
use common::{animal::{Animal, AnimalData}};
use reqwest::Client;

pub async fn get_animal_by_id(id: &str) -> Result<Animal> {
    let url = get_url("/api/animal/")?.join(id)?;
    
    let response = reqwest::get(url).await?;
    let parsed = response.json::<Option<Animal>>().await?;

    match parsed {
        Some(animal) => Ok(animal),
        None => bail!("No animal with given id")
    }
}

pub async fn get_all_animal() -> Result<Vec<Animal>> {
    let url = get_url("/api/animal-list")?;

    let response = reqwest::get(url).await?;
    let parsed = response.json::<Vec<Animal>>().await?;
    
    Ok(parsed)
}

pub async fn post_animal(animal: &AnimalData) -> Result<()> {
    let url = get_url("/api/animal")?;

    let client = Client::new();
    let _response = client.post(url)
        .json(&animal)
        .send()
        .await?;

    Ok(())
}