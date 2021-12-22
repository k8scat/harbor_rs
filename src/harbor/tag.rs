use super::client::Client;
use serde::{Deserialize, Serialize};
use anyhow::{Result, anyhow};
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
    pub scan_overview: Option<serde_json::Value>,
    pub config: Option<serde_json::Value>,
    pub push_time: String,
    pub pull_time: String,
    #[serde(rename = "os.version")]
    pub os_version: Option<String>,
}

impl Client {
    pub async fn list_tags(&self, repo_name: &str, label_id: Option<&str>, detail: Option<bool>) -> Result<Vec<Tag>> {
        let path = format!("repositories/{}/tags", repo_name);
        let req = self.get(path);
        let mut params = Vec::new();
        if let Some(label_id) = label_id {
            params.push(("label", label_id.to_string()));
        }
        if let Some(detail) = detail {
            params.push(("detail", detail.to_string()));
        }
        let resp = req.send().await?;
        let tags = resp.json::<Vec<Tag>>().await?;
        Ok(tags)
    }

    pub async fn delete_tag(&self, repo_name: &str, tag_name: &str) -> Result<()> {
        let path = format!("repositories/{}/tags/{}", repo_name, tag_name);
        let resp = self.delete(path).send().await?;
        if resp.status().is_success() {
            Ok(())
        } else {
            Err(anyhow!("{} {}", resp.status(), resp.text().await?))
        }
    }
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn test_list_tags() {

    }
}
