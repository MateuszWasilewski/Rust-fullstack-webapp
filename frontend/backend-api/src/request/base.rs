use anyhow::Result;
use dotenv_codegen::dotenv;
use serde::{de::DeserializeOwned, Serialize};
use wasm_utils::log;

#[cfg(debug_assertions)]
static BASE_URL: &str = dotenv!("LOCAL_SERVER_IP");
#[cfg(not(debug_assertions))]
static BASE_URL: &str = dotenv!("CLOUD_SERVER_IP");

fn get_url(input: &str) -> Result<reqwest::Url> {
    let url = reqwest::Url::parse(BASE_URL)?;
    let url = url.join(input)?;
    Ok(url)
}

async fn request<T, B>(method: reqwest::Method, url: &str, body: B) -> Result<T>
where
    T: DeserializeOwned + 'static,
    B: Serialize,
{
    let allow_body = method == reqwest::Method::POST || method == reqwest::Method::PUT;
    let url = get_url(url)?;
    let url_path = url.as_str();

    let mut builder = reqwest::Client::new().request(method, url.clone());
    if allow_body {
        builder = builder.json(&body);
    }

    let respose = builder.send().await?;
    log(&format!("Response for url: {} | received", url_path));
    let parsed = respose.json::<T>().await?;
    log(&format!("Response for url: {} | parsed", url_path));

    Ok(parsed)
}

pub async fn request_get<T>(url: &str) -> Result<T>
where
    T: DeserializeOwned + 'static,
{
    request(reqwest::Method::GET, url, ()).await
}

pub async fn request_post<T, B>(url: &str, body: B) -> Result<T>
where
    T: DeserializeOwned + 'static,
    B: Serialize,
{
    request(reqwest::Method::POST, url, body).await
}

pub async fn request_delete(url: &str) -> Result<()>
{
    request(reqwest::Method::DELETE, url, ()).await
}
