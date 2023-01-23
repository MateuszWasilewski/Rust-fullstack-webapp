use dotenv_codegen::dotenv;
use anyhow::Result;
use serde::{de::DeserializeOwned};

#[cfg(debug_assertions)]
static BASE_URL: &str = dotenv!("LOCAL_SERVER_IP");
#[cfg(not(debug_assertions))]
static BASE_URL: &str = dotenv!("CLOUD_SERVER_IP");

fn get_url(input: &str) -> Result<reqwest::Url> {
    let url = reqwest::Url::parse(BASE_URL)?;
    let url = url.join(input)?;
    Ok(url)
}

async fn request<T>(method: reqwest::Method, url: &str) -> Result<T> 
where
  T: DeserializeOwned + 'static
{
    let url = get_url(url)?;
    
    let builder = reqwest::Client::new()
      .request(method, url);

    let respose = builder.send().await?;
    let parsed = respose.json::<T>().await?;

    Ok(parsed)
}

pub async fn request_get<T>(url: &str) -> Result<T> 
where
  T: DeserializeOwned + 'static 
{
  request::<T>(reqwest::Method::GET, url).await
}
