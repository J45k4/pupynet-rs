use std::collections::HashMap;
use tokio::sync::mpsc;

use crate::protocol::STREAM_CONTINUE;
use crate::protocol::STREAM_END;
use crate::protocol::STREAM_START;

struct CurrentStream {
	stream_id: u64,
	length: u16,
	recv_count: u32,
}

pub struct Multiplex {
	streams: HashMap<u64, mpsc::Sender<Vec<u8>>>,
	current_stream: Option<CurrentStream>,
}

impl Multiplex {
	pub async fn run(mut self) {

	}

	pub async fn parse(&mut self, data: &[u8]) {
		match &mut self.current_stream {
			Some(stream_info) => {
				stream_info.recv_count += 1;
				let stream = match self.streams.get(&stream_info.stream_id) {
					Some(s) => s,
					None => return
				};
				let payload_left = stream_info.length as usize - stream_info.recv_count as usize;
				let payload = data[0..payload_left].to_vec();
				stream.send(payload).await.unwrap();
			},
			None => {
				let stream_id = u64::from_be_bytes(data[0..8].try_into().unwrap());
				let stage = (stream_id & 0x0F) as u8;
				let stream_id = stream_id >> 4;

				let stream = match stage {
					STREAM_START => {
						let (tx, rx) = mpsc::channel(128);
						tokio::spawn(async move {

						});
						tx
					}
					STREAM_END => {
						return;
					}
					STREAM_CONTINUE => {
						match self.streams.get(&stream_id) {
							Some(s) => s.to_owned(),
							None => return
						}
					}
					_ => {
						panic!("Invalid stage: {}", stage);
					}
				};

				let length: u16 = u16::from_be_bytes(data[8..10].try_into().unwrap());
				let payload_len = (data.len() - 10) as u16;

				if payload_len < length {
					self.current_stream = Some(CurrentStream {
						stream_id,
						length,
						recv_count: 0
					});
				}

				stream.send(data[10..].to_vec()).await.unwrap();
			}
		};
	}
}

// #[cfg(test)]
// mod tests {
// 	use super::*;

// 	#[tokio::test]
// 	pub async fn test_start_stream() {
// 		let mut multiplex = Multiplex {
// 			streams: HashMap::new(),
// 			current_stream: None
// 		};

// 	}
// }