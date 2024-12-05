use tokio::sync::broadcast;
use tokio::sync::mpsc;
use types::InternalCommand;
use worker::Worker;

mod ws;
mod types;
mod protocol;
mod worker;
mod udp;
mod stream;

#[derive(Debug, Clone)]
pub enum PupynetEvent {
	PeerConnected {
		addr: String
	},
	PeerDisconnected {
		addr: String
	},
	PeerData {
		addr: String,
		data: Vec<u8>
	}
}

pub struct Pupynet {
	tx: mpsc::UnboundedSender<InternalCommand>,
	event_tx: broadcast::Sender<PupynetEvent>,
	event_rx: broadcast::Receiver<PupynetEvent>,
}

impl Pupynet {
	pub fn new() -> Pupynet {
		let (event_tx, event_rx) = broadcast::channel(1024);
		let (tx, rx) = mpsc::unbounded_channel();
		{
			let event_tx = event_tx.clone();
			tokio::spawn(async move {
				Worker::new(rx, event_tx).await.run().await;
			});
		}

		Pupynet {
			tx,
			event_tx,
			event_rx
		}
	}

	pub fn bind(&self, addr: String) {

	}

	pub async fn next(&mut self) -> Option<PupynetEvent> {
		println!("next");
		None
	}
}