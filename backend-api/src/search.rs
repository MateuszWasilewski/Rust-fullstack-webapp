use anyhow::Result;
use common::SearchResult;

use crate::base::get_url;



pub async fn get_search_results(query: &str) -> Result<Vec<SearchResult>> {
  let url = get_url("/api/search/")?.join(query)?;

  let response = reqwest::get(url).await?;
  let parsed = response.json::<Vec<SearchResult>>().await?;

  Ok(parsed)
}