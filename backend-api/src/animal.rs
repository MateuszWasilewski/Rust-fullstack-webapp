use crate::base::get_url;
use anyhow::Result;
use common::{AnimalData};
use reqwest::Client;

pub async fn post_animal(animal: &AnimalData) -> Result<()> {
    let url = get_url("/api/animal")?;

    let client = Client::new();
    // TODO Change to use forms
    //let response = client.post(url.clone()).form(animal).send().await?;
    let response = client.post(url).json(&animal).send().await?;
    let parsed = response.json::<Option<()>>().await?;

    match parsed {
        None => bail!("Insertion failed"),
        Some(_) => Ok(()),
    }
}
