use super::client::Client;
use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Debug, Deserialize, Serialize)]
pub struct Schedule {
    #[serde(rename = "type")]
    pub schedule_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cron: Option<String>
}

impl Client {
    pub async fn create_schedule(&self) -> Result<()> {
        let url = self.build_api(String::from("system/gc/schedule"));
        let resp = self.client.post(url)
            .header("Content-Type", "application/json")
            .body(r#"{"schedule": {"type": "Manual"}}"#)
            .send()
            .await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::harbor::gc::Schedule;

    #[test]
    fn unmarshal_schedule() {
        let s = Schedule{
            schedule_type: "Manual".to_string(),
            cron: None
        };
        println!("{}", serde_json::to_string(&s).unwrap());
    }
}
