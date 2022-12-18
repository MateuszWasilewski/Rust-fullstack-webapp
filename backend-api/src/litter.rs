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
    let _response = client.post(url)
        .json(&litter)
        .send()
        .await?;
    Ok(())
}