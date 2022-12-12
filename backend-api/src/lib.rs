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

fn get_url(input: &str) -> Result<reqwest::Url> {
    let url = reqwest::Url::parse(BASE_URL)?;
    let url = url.join(input)?;
    Ok(url)
}

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

pub async fn get_genes() -> Result<Vec<String>> {
    let url = get_url("/api/genes-list")?;

    let response = reqwest::get(url).await?;
    let parsed = response.json::<Vec<String>>().await?;

    Ok(parsed)
}

pub async fn get_phenotypes() -> Result<Vec<Phenotype>> {
    let url = get_url("/api/phenotype-list")?;

    let response = reqwest::get(url).await?;
    let parsed = response.json::<Vec<Phenotype>>().await?;

    Ok(parsed)
}