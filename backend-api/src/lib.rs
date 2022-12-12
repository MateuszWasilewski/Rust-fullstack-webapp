use common::{animal::{Animal, AnimalStatus, Litter, Gender}, Photo, Phenotype};
use chrono::NaiveDate;
use anyhow::Result;
use dotenv_codegen::dotenv;

#[macro_use]
extern crate anyhow;

#[cfg(debug_assertions)]
static BASE_URL: &str = dotenv!("LOCAL_SERVER_IP");
#[cfg(not(debug_assertions))]
static BASE_URL: &str = dotenv!("CLOUD_SERVER_IP");

pub async fn get_animal_by_id(id: &str) -> Result<Animal> {
    let animals = get_all_animal().await?;
    for animal in animals {
        if animal.id == id {
            return Ok(animal)
        }
    }
    Err(anyhow!("No animal with given id"))
}

pub async fn get_all_animal() -> Result<Vec<Animal>> {
    let url = reqwest::Url::parse(BASE_URL)?;
    let url = url.join("/api/animal-list")?;

    let response = reqwest::get(url).await?;
    let parsed = response.json::<Vec<Animal>>().await?;
    
    Ok(parsed)
}

pub async fn get_genes() -> Result<Vec<String>> {
    let url = reqwest::Url::parse(BASE_URL)?;
    let url = url.join("/api/genes-list")?;

    let response = reqwest::get(url).await?;
    let parsed = response.json::<Vec<String>>().await?;

    Ok(parsed)
}

pub async fn get_phenotypes() -> Result<Vec<Phenotype>> {
    let url = reqwest::Url::parse(BASE_URL)?;
    let url = url.join("/api/phenotype-list")?;

    let response = reqwest::get(url).await?;
    let parsed = response.json::<Vec<Phenotype>>().await?;

    Ok(parsed)
}