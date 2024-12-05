use tokio::io::AsyncReadExt;
use tokio::io::AsyncSeekExt;
use tokio::io::AsyncWriteExt;
use tokio::io::BufReader;

use crate::protocol::CMD_READ_FILE;
use crate::protocol::CMD_REMOVE;
use crate::protocol::CMD_WRITE_FILE;

// struct Parser {
// 	stream_id: u64,
// 	streams: 
// }

// impl Parser {

// }

struct Stream {

}

impl Stream {
	pub async fn read_u16(&mut self) -> u16 {
		0
	}

	pub async fn read_u64(&mut self) -> u64 {
		0
	}

	pub async fn read_str(&mut self) -> &str {
		"lol"
	}

	pub async fn read_bytes(&mut self) -> Option<&[u8]> {
		Some(&[])
	}

	pub async fn write_bytes(&mut self, data: &[u8]) {
	}
}

async fn handle_stream(mut stream: Stream) -> anyhow::Result<()> {
	let cmd_type = stream.read_u16().await;

	match cmd_type {
		// INTRODUCE_CMD => {
		// 	let id = stream.read_str();
		// 	let name = stream.read_str();
		// }
		CMD_READ_FILE => {
			let mut file = {
				let path = stream.read_str().await;
				tokio::fs::OpenOptions::new().read(true).open(path).await.unwrap()
			};
			let offset = stream.read_u64().await;
			let length = stream.read_u64().await;
			file.seek(std::io::SeekFrom::Start(offset)).await.unwrap();
			let mut reader = BufReader::new(file);
			let mut buff = [0u8; 8192];
			let mut total_read = 0;
			while total_read < length {
				let to_read = std::cmp::min(buff.len() as u64, length - total_read) as usize;
				let read = reader.read(&mut buff[..to_read]).await.unwrap();
				if read == 0 {
					break;
				}
				stream.write_bytes(&buff[..read]).await;
				total_read += read as u64;
			}
		}
		CMD_WRITE_FILE => {
			let mut file = {
				let path = stream.read_str().await;
				tokio::fs::OpenOptions::new().write(true).create(true).open(path).await.unwrap()
			};
			let offset = stream.read_u64().await;
			file.seek(std::io::SeekFrom::Start(offset)).await.unwrap();
			while let Some(data) = stream.read_bytes().await {
				file.write_all(data).await.unwrap();
			}
		}
		CMD_REMOVE => {
			let path = stream.read_str().await.to_string();
			let metadata = tokio::fs::metadata(&path).await?;

			
		}
		_ => {}
	}

	Ok(())
}