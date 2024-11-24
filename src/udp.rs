use std::sync::Arc;

use tokio::net::UdpSocket;
use tokio::sync::mpsc;

use crate::protocol::PupynetParser;
use crate::types::InternalEvent;
use crate::PupynetEvent;

fn process_socket(socket: Arc<UdpSocket>, tx: mpsc::UnboundedSender<InternalEvent>) {
	tokio::spawn(async move {
		let mut parser = PupynetParser::new();
		loop {
			let mut buf = [0; 1024];
			let (len, addr) = socket.recv_from(&mut buf).await.unwrap();
			log::info!("received {} bytes from {}", len, addr);
			parser.parse(&buf[0..len]);
			while let Some(cmd) = parser.next() {
				let addr = format!("udp://{}", addr);
				match tx.send(InternalEvent::PeerCmd { addr, cmd }) {
					Ok(_) => {},
					Err(err) => {
						log::error!("error sending event: {}", err);
						return;
					}
				}
			}
		}
	});
}

pub async fn bind_default(tx: mpsc::UnboundedSender<InternalEvent>) -> Arc<UdpSocket> {
	match UdpSocket::bind("0.0.0.0:7764").await {
		Result::Ok(socket) => {
			log::info!("bound broadcast socket");
			let socket: Arc<UdpSocket> = Arc::new(socket);
			let tx = tx.clone();
			socket.set_broadcast(true).unwrap();
			{
				let broadcast_socket = socket.clone();
				process_socket(broadcast_socket, tx);
			}
			socket
		},
		Err(err) => {
			log::error!("error binding broadcast socket: {}", err);
			let socket = UdpSocket::bind("0.0.0.0:0").await.unwrap();
			socket.set_broadcast(true).unwrap();
			Arc::new(socket)
		},
	}
}