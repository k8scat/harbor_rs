use std::str::FromStr;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use anyhow::{anyhow, Result};
use reqwest::{IntoUrl, Method};

#[derive(Debug)]
pub struct Client {
    base_url: String,
    pub client: reqwest::Client,
}

impl Client {
    pub fn new(base_url: String, username: String, password: String) -> Result<Client> {
        let token = base64::encode(format!("{}:{}", username, password));
        let client = Client {
            base_url,
            client: reqwest::Client::builder()
                .timeout(core::time::Duration::from_secs(60))
                .default_headers(
                    HeaderMap::from_iter(vec![
                        (HeaderName::from_str("Authorization")?,
                         HeaderValue::from_str(format!("Basic {}", token).as_str())?),
                    ]),
                )
                .build()?,
        };
        Ok(client)
    }

    pub fn setup() -> Result<Client> {
        let base_url: &'static str = option_env!("HARBOR_BASE_URL").ok_or(anyhow!("HARBOR_BASE_URL not set"))?;
        let username: &'static str = option_env!("HARBOR_USERNAME").ok_or(anyhow!("HARBOR_USERNAME not set"))?;
        let password: &'static str = option_env!("HARBOR_PASSWORD").ok_or(anyhow!("HARBOR_PASSWORD not set"))?;
        Ok(self::Client::new(
            base_url.to_string(),
            username.to_string(),
            password.to_string())?)
    }

    pub fn build_request<U: IntoUrl>(&self, method: Method, path: U) -> reqwest::RequestBuilder {
        let url = self.build_url(path);
        self.client.request(method, url)
    }

    fn build_url<U: IntoUrl>(&self, path: U) -> String {
        format!("{}{}", self.base_url, path.as_str())
    }
}

