use crate::base::get_url;
use anyhow::Result;
use common::{AnimalFull, AnimalData};
use reqwest::Client;

pub async fn get_animal_by_id(id: &str) -> Result<AnimalFull> {
    let url = get_url("/api/animal/")?.join(id)?;
    
    let response = reqwest::get(url).await?;
    let parsed = response.json::<Option<AnimalFull>>().await?;

    match parsed {
        None => bail!("No animal with given id"),
        Some(animal) => Ok(animal),
    }
}

pub async fn get_all_animal() -> Result<Vec<AnimalData>> {
    let url = get_url("/api/animal-list")?;

    let response = reqwest::get(url).await?;
    let parsed = response.json::<Option<Vec<AnimalData>>>().await?;

    match parsed {
        None => bail!("no data send by backend"),
        Some(animals) => Ok(animals)
    }
}

pub async fn post_animal(animal: &AnimalData) -> Result<()> {
    let url = get_url("/api/animal")?;

    let client = Client::new();
    let response = client.post(url)
        .json(&animal)
        .send()
        .await?;
    let parsed = response.json::<Option<()>>().await?;

    match parsed {
        None => bail!("Insertion failed"),
        Some(_) => Ok(())
    }
}