use std::str::FromStr;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use anyhow::Result;

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


    pub fn build_api(&self, path: String) -> String {
        format!("{}/{}", self.base_url, path)
    }
}

