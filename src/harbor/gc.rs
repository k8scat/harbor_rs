use std::fmt;
use super::client::Client;
use serde::{Deserialize, Serialize};
use anyhow::{anyhow, Result};
use reqwest::Method;
use serde_json::json;

#[derive(Debug)]
pub enum ScheduleType {
    Hourly,
    Daily,
    Weekly,
    Custom,
    Manual,
    None
}

/// enum to String
///
/// ```rust
/// ScheduleType::Hourly.to_string() // "Hourly"
/// ```
impl fmt::Display for ScheduleType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Schedule {
    #[serde(rename = "type")]
    pub schedule_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cron: Option<String>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GCResult {
    pub job_status: String,
    pub update_time: String,
    pub schedule: Schedule,
    pub deleted: bool,
    pub job_kind: String,
    pub creation_time: String,
    pub id: u64,
    pub job_name: String
}

impl Client {
    /// Create a gc schedule.
    pub async fn create_schedule(&self, schedule: &Schedule) -> Result<()> {
        let path = "/system/gc/schedule";
        let payload = json!({"schedule": schedule});
        let resp = self.build_request(Method::POST, path)
            .json(&payload)
            .send()
            .await?;
        if resp.status().eq(&reqwest::StatusCode::CREATED) {
            Ok(())
        } else {
            Err(anyhow!("failed to create schedule: {}", resp.text().await?))
        }
    }

    /// Get gc results.
    pub async fn list_gc_results(&self) -> Result<Vec<GCResult>> {
        let path = "/system/gc";
        let resp = self.build_request(Method::GET, path).send().await?;
        Ok(resp.json::<Vec<GCResult>>().await?)
    }

    /// Get gc status.
    pub async fn get_gc_result(&self, id: u64) -> Result<GCResult> {
        let path = format!("/system/gc/{}", id);
        let resp = self.build_request(Method::GET, path).send().await?;
        Ok(resp.json::<GCResult>().await?)
    }
}
