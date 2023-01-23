use anyhow::Result;
use dotenv_codegen::dotenv;

#[cfg(debug_assertions)]
static BASE_URL: &str = dotenv!("LOCAL_SERVER_IP");
#[cfg(not(debug_assertions))]
static BASE_URL: &str = dotenv!("CLOUD_SERVER_IP");

pub fn get_url(input: &str) -> Result<reqwest::Url> {
    let url = reqwest::Url::parse(BASE_URL)?;
    let url = url.join(input)?;
    Ok(url)
}
