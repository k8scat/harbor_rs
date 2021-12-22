use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Signature {
    pub description: String
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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

pub fn parse_time(time: &str) -> Result<DateTime<Utc>> {
    Ok(time.parse::<DateTime<Utc>>()?)
}

#[cfg(test)]
mod tests {
    use crate::harbor::common::parse_time;

    #[test]
    fn test_parse_time() {
        struct Date {
            s: &'static str,
            c: &'static str,
        }
        let data = vec![
            Date { s: "2021-12-02T04:35:12.923959Z", c: "2021-12-02 04:35:12" },
            Date { s: "2021-12-02T09:34:39.358084913Z", c: "2021-12-02 09:34:39" },
        ];
        for d in data {
            let t = parse_time(d.s).unwrap();
            assert_eq!(t.format("%Y-%m-%d %H:%M:%S").to_string(), d.c);
        }
    }
}
