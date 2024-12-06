use std::env::home_dir;
use std::path::Path;
use std::path::PathBuf;

use anyhow::bail;
use args::Args;
use args::Command;
use pupynet_core::Pupynet;
use clap::Parser;
use reqwest::header;
use serde_json::Value;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

mod args;

pub fn get_version() -> u32 {
    match option_env!("VERSION") {
        Some(ver) => {
            ver.parse().unwrap_or(0)
        },
        None => {
            0
        }
    }
}

fn get_os_name() -> String {
	let os = std::env::consts::OS;
	let arch = std::env::consts::ARCH;
	format!("{}", os)
}

fn app_dir() -> PathBuf {
	let path = homedir::my_home().unwrap().unwrap().join(".pupynet");
	if !path.exists() {
		std::fs::create_dir_all(&path).unwrap();
	}
	path
}

async fn fetch_latest_release() -> anyhow::Result<()> {
    let client = reqwest::Client::new();
    let res = client
        .get("https://api.github.com/repos/j45k4/pupynet-rs/releases/latest")
        .header("User-Agent", "pupynet")
        .send()
        .await?
        .text()
        .await?;
    
    let res: Value = serde_json::from_str(&res)?;

	let tag = res["tag_name"].as_str().unwrap();
	log::info!("tag: {}", tag);
	let tag = tag.parse::<u32>().unwrap();
	let current = get_version();
	
	log::info!("current: {}", current);
	log::info!("latest: {}", tag);

	if tag <= current {
		log::info!("Already up to date");
		return Ok(());
	}

    let assets = match res["assets"] {
        Value::Array(ref assets) => assets,
        _ => bail!("no assets found"),
    };

    let os_name = get_os_name();
    let asset = match assets.iter().find(|asset| {
        if let Some(name) = asset["name"].as_str() {
            name.contains(&os_name)
        } else {
            false
        }
    }) {
        Some(asset) => asset,
        None => bail!("no asset found for os: {}", os_name),
    };

    let download_url = asset["browser_download_url"]
        .as_str()
        .ok_or_else(|| anyhow::anyhow!("no download url found"))?;

    log::info!("download_url: {}", download_url);

    // Attempt to derive a local filename from the asset name
    let filename = asset["name"]
        .as_str()
        .map(|s| s.to_string())
        .unwrap_or_else(|| "downloaded_binary".to_string());

    log::info!("Downloading asset: {}", filename);

    let download_res = client.get(download_url).send().await?;
    if !download_res.status().is_success() {
        bail!("Failed to download asset. HTTP status: {}", download_res.status());
    }

    let bytes = download_res.bytes().await?;
    // Save the file
	let path = app_dir().join(&filename);
    let mut file = File::create(&path).await?;
    file.write_all(&bytes).await?;

    log::info!("Successfully downloaded and saved: {}", path.display());

    Ok(())
}

#[tokio::main]
async fn main() {
	simple_logger::init_with_level(log::Level::Info).unwrap();

	let args = Args::parse();

	match args.cmd {
		Some(Command::Update) => {
			fetch_latest_release().await.unwrap();
		}
		_ => {
			
		}
	}
}
