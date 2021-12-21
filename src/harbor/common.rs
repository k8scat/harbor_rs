use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Debug, Deserialize, Serialize)]
pub struct Signature {
    pub description: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Label {
    pub description: String,
    pub update_time: String,
    pub color: String,
    pub creation_time: String,
    pub deleted: bool,
    pub scope: String,
    pub project_id: u64,
    pub id: u64,
    pub name: String
}

pub fn parse_time(time: String) -> Result<DateTime<Local>> {
    Ok(time.parse::<DateTime<Local>>()?)
}

