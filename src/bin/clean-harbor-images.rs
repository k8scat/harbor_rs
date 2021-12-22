use std::io::Read;
use std::ops::Sub;
use chrono::Duration;
use chrono::prelude::*;
use anyhow::{anyhow, Result};
use clap::{App, Arg};
use harbor_rs::Client;
use serde::{Deserialize, Serialize};
use harbor_rs::harbor::common::parse_time;
use harbor_rs::harbor::gc::{Schedule, ScheduleType};

extern crate base64;

#[derive(Debug, Deserialize, Serialize)]
struct Config {
    pub harbor_base_api: String,
    pub harbor_username: String,
    pub harbor_password: String,
    pub clean_interval: u32,
    pub repos: Vec<String>,
}

#[tokio::main]
async fn main() {
    let matches = App::new("clean-harbor-images")
        .version("1.0")
        .author("K8sCat <rustpanic@gmail.com>")
        .about("Clean harbor images")
        .arg(Arg::with_name("config")
            .short("c")
            .long("config")
            .value_name("FILE")
            .default_value("config.yml")
            .help("Set config file")
            .takes_value(true))
        .get_matches();
    let config_file = matches.value_of("config").unwrap();
    let config = load_config(config_file).unwrap();
    let client = harbor_rs::Client::new(config.harbor_base_api, config.harbor_username, config.harbor_password).unwrap();
    let clean_interval = Local::now().sub(Duration::days(config.clean_interval as i64));
    for repo in config.repos {
        clean(&client, repo.as_str(), clean_interval).await.unwrap();
    }
}

fn load_config(path: &str) -> Result<Config> {
    let mut f = std::fs::File::open(path)?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    let config: Config = serde_yaml::from_str(&s)?;
    Ok(config)
}

async fn clean(client: &Client, repo: &str, interval: DateTime<Local>) -> Result<()> {
    let tags = client.list_tags(repo, None, Some(true)).await?;
    for tag in tags {
        let push_time = parse_time(tag.push_time.as_str())?;
        if push_time.le(&interval) {
            client.delete_tag(repo, tag.name.as_str()).await?;
            println!("deleted {} which pushed at {}", tag.name, push_time.format("%Y-%m-%d %H:%M:%S"));
        }
    }
    Ok(manual_gc(client).await?)
}

async fn manual_gc(client: &Client) -> Result<()> {
    let schedule = Schedule {
        schedule_type: ScheduleType::Manual.to_string(),
        cron: None,
    };
    client.create_schedule(&schedule).await?;
    println!("manual gc schedule created");

    for _ in 1..100 {
        let gc_results = client.list_gc_results().await?;
        if gc_results.len() == 0 {
            return Err(anyhow!("gc results is empty"));
        }
        let gc_result = gc_results.get(0).unwrap();
        if gc_result.job_status == "finished" {
            println!("gc status: finished");
            return Ok(());
        }
        println!("gc status: {}", gc_result.job_status);
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
    Err(anyhow!("gc is still not finished, please check manually"))
}

