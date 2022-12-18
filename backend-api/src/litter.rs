use anyhow::Result;

use common::litter::LitterData;
use crate::base::get_url;


pub async fn get_litter_list() -> Result<Vec<LitterData>> {
    let url = get_url("/api/litter-list")?;

    let response = reqwest::get(url).await?;
    let parsed = response.json::<Vec<LitterData>>().await?;
    
    Ok(parsed)
}