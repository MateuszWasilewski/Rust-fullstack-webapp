use crate::base::get_url;
use anyhow::Result;
use common::{AnimalData};
use reqwest::Client;

pub async fn post_animal(animal: &AnimalData) -> Result<()> {
    let url = get_url(&format!("/api/animal/{}", animal.id))?;

    let client = Client::new();
    // TODO Change to use forms
    //let response = client.post(url.clone()).form(animal).send().await?;
    let _response = client.post(url).json(&animal).send().await?;
    Ok(())
}
