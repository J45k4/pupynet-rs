use std::env::home_dir;
use std::io::BufReader;
use std::path::Path;
use std::path::PathBuf;

use anyhow::bail;
use args::Args;
use args::Command;
use flate2::bufread::GzDecoder;
use pupynet_core::Pupynet;
use clap::Parser;
use reqwest::header;
use reqwest::Url;
use rsa::pkcs1v15;
use rsa::pkcs1v15::Signature;
use rsa::pkcs1v15::VerifyingKey;
use rsa::pkcs8::DecodePublicKey;
use rsa::RsaPublicKey;
use serde_json::Value;
use sha2::Sha256;
use tar::Archive;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use rsa::signature::Verifier;
use tokio::sync::mpsc;

mod args;

pub const PUBLIC_KEY: &str = include_str!("../../public_key.pem");

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

pub fn verify_signature(bin: &Path, sig: &Path) -> anyhow::Result<bool> {
	log::info!("verifying {} with {}", bin.display(), sig.display());
	let public_key = RsaPublicKey::from_public_key_pem(PUBLIC_KEY).unwrap();
	let verifying_key = pkcs1v15::VerifyingKey::<Sha256>::new(public_key);
	let signature = std::fs::read(sig)?;
	let signature = rsa::pkcs1v15::Signature::try_from(signature.as_slice())?;
	let data = std::fs::read(bin)?;
	Ok(verifying_key.verify(&data, &signature).is_ok())
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

fn bin_dir() -> PathBuf {
    let path = app_dir().join("bin");
    if !path.exists() {
        std::fs::create_dir_all(&path).unwrap();
    }
    path
}

async fn fetch_latest_release() -> anyhow::Result<Value> {
    let client = reqwest::Client::new();
    let res = client
        .get("https://api.github.com/repos/j45k4/pupynet-rs/releases/latest")
        .header("User-Agent", "pupynet")
        .send()
        .await?
        .text()
        .await?;
    
    Ok(serde_json::from_str::<Value>(&res)?)
}

async fn dowload_bin(url: &str, filename: &str) -> anyhow::Result<PathBuf> {
    let res = reqwest::get(url).await?;
    if !res.status().is_success() {
        bail!("Failed to download asset. HTTP status: {}", res.status());
    }
    let bytes = res.bytes().await?;
    let path = app_dir().join(&filename);
    let mut file = File::create(&path).await?;
    file.write_all(&bytes).await?;
    Ok(path)
}

async fn update_bin() -> anyhow::Result<()> {
	let res = fetch_latest_release().await?;
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

	let path = dowload_bin(download_url, &filename).await?;

	log::info!("Downloaded asset to: {:?}", path);

    let file = std::fs::File::open(path)?;
    let buf_reader = BufReader::new(file);
    let decoder = GzDecoder::new(buf_reader);
    let mut archive = Archive::new(decoder);
	let mut entries = archive.entries()?;
	while let Some(file) = entries.next() {
		let mut file = file?;
		let name = match file.path() {
			Ok(name) => name,
			Err(_) => continue,
		};
		log::info!("unpacking: {:?}", name);
		let dst = app_dir().join(name);
		log::info!("unpacking to {:?}", dst);
		file.unpack(dst)?;
	}
	let bin_path = app_dir().join("pupynet");
	let sig_path = bin_path.with_extension("sig");
	if !verify_signature(&bin_path, &sig_path)? {
		bail!("Signature verification failed");
	}
	tokio::fs::copy(&bin_path, bin_dir().join("pupynet")).await?;
	tokio::fs::remove_file(&bin_path).await?;
	tokio::fs::remove_file(&sig_path).await?;
	Ok(())
}

#[tokio::main]
async fn main() {
	simple_logger::init_with_level(log::Level::Info).unwrap();
	log::info!("Pupynet version: {}", get_version());

	let args = Args::parse();

	match args.cmd {
		Some(Command::Update) => {
			update_bin().await.unwrap();
		}
		Some(Command::Verify { bin, sig }) => {
			let bin_path = Path::new(&bin);
			let sig_path = Path::new(&sig);
			if verify_signature(bin_path, sig_path).unwrap() {
				log::info!("Signature verification passed");
			} else {
				log::error!("Signature verification failed");
			}
		}
		_ => {
			
		}
	}

	let (tx, mut rx) = mpsc::channel::<[u8; 1024]>(10); // 10 slots for fixed-size arrays

    tokio::spawn(async move {
        for i in 0..20 {
            let data = [i as u8; 1024]; // Reuse static allocation
            tx.send(data).await.unwrap();
        }
    });

    while let Some(data) = rx.recv().await {
        println!("Received: {:?}", data[0]); // Prints first byte of each message
    }
}
