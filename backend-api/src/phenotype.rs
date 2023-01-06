use anyhow::Result;
use common::Phenotype;

use crate::base::get_url;

pub async fn get_genes() -> Result<Vec<String>> {
    let url = get_url("/api/genes-list")?;

    let response = reqwest::get(url).await?;
    let parsed = response.json::<Vec<String>>().await?;

    Ok(parsed)
}

pub async fn get_phenotypes() -> Result<Vec<Phenotype>> {
    let url = get_url("/api/simple-phenotype-list")?;

    let response = reqwest::get(url).await?;
    let parsed = response.json::<Vec<Phenotype>>().await?;

    Ok(parsed)
}

pub async fn get_phenotypes_full() -> Result<Vec<Phenotype>> {
    let url = get_url("/api/phenotype-list")?;

    let response = reqwest::get(url).await?;
    let parsed = response.json::<Vec<Phenotype>>().await?;

    Ok(parsed)
}
