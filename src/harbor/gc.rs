use std::fmt;
use super::client::Client;
use serde::{Deserialize, Serialize};
use anyhow::{anyhow, Result};

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
    pub id: u32,
    pub job_name: String
}

impl Client {
    pub async fn create_schedule(&self, schedule: &Schedule) -> Result<()> {
        let url = self.build_api(String::from("system/gc/schedule"));
        let resp = self.client.post(url)
            .header("Content-Type", "application/json")
            .body(format!("{{\"schedule\": {}}}", serde_json::to_string(schedule)?))
            .send()
            .await?;
        if resp.status().eq(&reqwest::StatusCode::CREATED) {
            Ok(())
        } else {
            Err(anyhow!("{} {}", resp.status(), resp.text().await?))
        }
    }

    pub async fn list_gc_results(&self) -> Result<Vec<GCResult>> {
        let url = self.build_api(String::from("system/gc"));
        let resp = self.client.get(url).send().await?;
        let results = resp.json::<Vec<GCResult>>().await?;
        Ok(results)
    }
}
