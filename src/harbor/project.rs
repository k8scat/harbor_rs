use serde_json::Value;
use serde::{Deserialize, Serialize};
use crate::Client;
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub update_time: String,
    pub owner_name: String,
    pub name: String,
    pub deleted: bool,
    pub owner_id: i64,
    pub repo_count: i64,
    pub creation_time: String,
    pub togglable: Option<bool>,
    pub project_id: i64,
    pub current_user_role_id: i64,
    pub chart_count: i64,
    pub metadata: Option<Value>,
    pub cve_whitelist: Option<Value>,
    pub current_user_role_ids: Vec<i64>
}

/// The webhook job.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookJob {
    /// The webhook job status.
    pub status: String,
    /// The webhook job update time.
    pub update_time: String,
    /// The webhook job event type.
    pub event_type: String,
    /// The webhook job creation time.
    pub creation_time: String,
    /// The webhook job notify detailed data.
    pub job_detail: String,
    /// The webhook job ID.
    pub id: i64,
    /// The webhook job notify type.
    pub notify_type: String,
    /// The webhook policy ID.
    pub policy_id: i64,
}

impl Client {
    /// List projects
    pub async fn list_projects(&self, name: Option<String>, public: Option<bool>, owner: Option<String>, page: Option<u32>, page_size: Option<u32>) -> Result<Vec<Project>> {
        let path = "projects";
        let mut params = Vec::new();
        if let Some(name) = name {
            params.push(("name", name));
        }
        if let Some(public) = public {
            params.push(("public", public.to_string()));
        }
        if let Some(owner) = owner {
            params.push(("owner", owner));
        }
        if let Some(page) = page {
            params.push(("page", page.to_string()));
        }
        if let Some(page_size) = page_size {
            params.push(("page_size", page_size.to_string()));
        }
        let resp = self.build_request(reqwest::Method::GET, path).query(&params).send().await?;
        if resp.status() != reqwest::StatusCode::OK {
            return Err(anyhow::anyhow!("failed to list projects: {}", resp.text().await?));
        }
        // handle null response
        let projects = resp.json::<Vec<Project>>().await;
        match projects {
            Ok(projects) => Ok(projects),
            Err(_) => Ok(vec![])
        }
    }

    /// List project webhook jobs
    pub async fn list_webhook_jobs(&self, project_id: i64, policy_id: i64) -> Result<Vec<WebhookJob>> {
        let path = format!("/projects/{}/webhook/jobs", project_id);
        let params = [("policy_id", policy_id.to_string())];
        let resp = self.build_request(reqwest::Method::GET, path).query(&params).send().await?;
        if resp.status() != reqwest::StatusCode::OK {
            return Err(anyhow::anyhow!("failed to list webhook jobs: {}", resp.text().await?));
        }
        let webhook_jobs = resp.json::<Vec<WebhookJob>>().await?;
        Ok(webhook_jobs)
    }

    /// Delete project by projectID
    pub async fn delete_project(&self, id: i64) -> Result<()> {
        let path = format!("/projects/{}", id);
        let resp = self.build_request(reqwest::Method::DELETE, path).send().await?;
        if resp.status() != reqwest::StatusCode::NO_CONTENT {
            return Err(anyhow::anyhow!("failed to delete project: {}", resp.text().await?));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn list_projects() {
        let client = crate::Client::setup().unwrap();
        client.list_projects(Some(String::from("test")), None, None, None, None).await.unwrap();
    }
}
