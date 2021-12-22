use super::client::Client;
use serde::{Deserialize, Serialize};
use anyhow::{Result, anyhow};
use reqwest::{Method, StatusCode};
use serde_json::Value;
use crate::harbor::common::Label;
use super::common::Signature;

#[derive(Debug, Deserialize, Serialize)]
pub struct Tag {
    pub name: String,
    pub author: String,
    pub docker_version: String,
    pub created: String,
    pub architecture: String,
    pub os: String,
    pub digest: String,
    pub size: u64,
    pub signature: Option<Signature>,
    pub labels: Vec<Label>,
    pub scan_overview: Option<Value>,
    pub config: Option<Value>,
    pub push_time: String,
    pub pull_time: String,
    #[serde(rename = "os.version")]
    pub os_version: Option<String>,
}

impl Client {
    /// Get tags of a relevant repository.
    pub async fn list_tags(&self, repo_name: &str, label_id: Option<&str>, detail: Option<bool>) -> Result<Vec<Tag>> {
        let path = format!("/repositories/{}/tags", repo_name);
        let mut params = Vec::new();
        if let Some(label_id) = label_id {
            params.push(("label", label_id.to_string()));
        }
        if let Some(detail) = detail {
            params.push(("detail", detail.to_string()));
        }
        let resp = self.build_request(Method::GET, path).query(&params).send().await?;
        if resp.status().eq(&StatusCode::OK) {
            Ok(resp.json::<Vec<Tag>>().await?)
        } else {
            Err(anyhow!("failed to list tags: {}", resp.text().await?))
        }
    }

    /// Delete a tag in a repository.
    pub async fn delete_tag(&self, repo_name: &str, tag_name: &str) -> Result<()> {
        let path = format!("/repositories/{}/tags/{}", repo_name, tag_name);
        let resp = self.build_request(Method::DELETE, path).send().await?;
        if resp.status().eq(&reqwest::StatusCode::OK) {
            Ok(())
        } else {
            Err(anyhow!("failed to delete tag: {}", resp.text().await?))
        }
    }
}
