use anyhow::Result;

use common::litter::LitterData;
use reqwest::Client;
use crate::base::get_url;


pub async fn get_litter_list() -> Result<Vec<LitterData>> {
    let url = get_url("/api/litter-list")?;

    let response = reqwest::get(url).await?;
    let parsed = response.json::<Vec<LitterData>>().await?;
    
    Ok(parsed)
}

pub async fn post_litter(litter: &LitterData) -> Result<()> {
    let url = get_url("/api/litter")?;

    let client = Client::new();
    let response = client.post(url)
        .json(&litter)
        .send()
        .await?;

    let parsed = response.json::<Option<()>>().await?;
    match parsed {
        Some(_) => Ok(()),
        None => bail!("Insertion has failed")
    }
}