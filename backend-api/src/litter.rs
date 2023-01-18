use anyhow::Result;

use crate::base::get_url;
use common::{litter::LitterData};
use reqwest::Client;

pub async fn post_litter(litter: &LitterData) -> Result<()> {
    let url = get_url("/api/litter")?;

    let client = Client::new();
    let response = client.post(url).json(&litter).send().await?;

    let parsed = response.json::<Option<()>>().await?;
    match parsed {
        Some(_) => Ok(()),
        None => bail!("Insertion has failed"),
    }
}
