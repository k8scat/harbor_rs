use std::collections::HashMap;
use std::ops::Sub;
use std::str::FromStr;
use chrono::Duration;
use chrono::prelude::*;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};

extern crate base64;

const HARBOR_BASE_API: &str = "https://dev-private.myones.net/api";
const REPO_NAME: &str = "test/ones-release";
const HARBOR_USERNAME: &str = "admin";
const HARBOR_PASSWORD: &str = "Harbor12345";

// #[tokio::main]
// fn main() {
//     let t = Local::now().sub(Duration::days(20));
//     delete_tags(&t).await;
// }

fn build_api(path: &str) -> String {
    format!("{}/{}", HARBOR_BASE_API, path)
}

async fn delete_tags(t: &DateTime<Local>) -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::builder()
        .timeout(core::time::Duration::from_secs(10))
        .default_headers(
            HeaderMap::from_iter(vec![
                (HeaderName::from_str("Authorization")?,
                 HeaderValue::from_str(format!("Basic {}", base64::encode(format!("{}:{}", HARBOR_USERNAME, HARBOR_PASSWORD))).as_str()).unwrap()),
            ]),
        )
        .build()?;

    let resp = client.get(build_api(format!("repositories/{}/tags?detail=true", REPO_NAME).as_str()))
        .send()
        .await?;
    if !resp.status().is_success() {
        panic!("failed to get tags");
    }
    let tags = resp.json::<Vec<HashMap<String, serde_json::Value>>>().await?;
    for tag in tags.iter() {
        let name = tag.get("name").unwrap();
        let push_time = tag.get("push_time").unwrap();
        let push_time = push_time.as_str().unwrap().parse::<DateTime<Local>>().unwrap();
        if push_time.le(t) {
            let name = name.as_str().unwrap();
            println!("delete {} which pushed at {}", name, push_time.format("%Y-%m-%d %H:%M:%S"));
            let resp = client.delete(build_api(format!("repositories/{}/tags/{}", REPO_NAME, name).as_str()))
                .send()
                .await?;
            if resp.status().is_success() {
                println!("delete {} success", name);
            } else {
                println!("delete {} failed: {}", name, resp.status());
            }
        }
    }

    println!("wait for deleting tags");
    std::thread::sleep(std::time::Duration::from_secs(5));

    let resp = client.post(build_api("system/gc/schedule"))
        .header("Content-Type", "application/json")
        .body(r#"{"schedule": {"type": "Manual"}}"#)
        .send()
        .await?;
    if !resp.status().is_success() {
        panic!("failed to schedule gc");
    }
    println!("gc scheduled");

    loop {
        let resp = client.get(build_api("system/gc"))
            .send()
            .await?;
        if !resp.status().is_success() {
            panic!("failed to get gc results");
        }
        let results = resp.json::<Vec<HashMap<String, serde_json::Value>>>().await?;
        if results.len() == 0 {
            panic!("gc results is empty");
        }
        let result = results.get(0).unwrap();
        let status = result.get("job_status").unwrap().as_str().unwrap();
        if status == "finished" {
            println!("gc finished");
            break;
        }
        println!("gc status: {}", status);
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
    Ok(())
}

