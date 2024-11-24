use std::collections::HashMap;

use tokio::sync::broadcast;
use tokio::sync::mpsc;

use crate::protocol::PeerCmd;
use crate::types::InternalCommand;
use crate::types::InternalEvent;
use crate::PupynetEvent;


pub struct Worker {
	event_tx: broadcast::Sender<PupynetEvent>,
	internal_event_tx: mpsc::UnboundedSender<InternalEvent>,
	internal_event_rx: mpsc::UnboundedReceiver<InternalEvent>,
	rx: mpsc::UnboundedReceiver<InternalCommand>

}

impl Worker {
	pub fn new(rx: mpsc::UnboundedReceiver<InternalCommand>, event_tx: broadcast::Sender<PupynetEvent>) -> Self {
		let (internal_event_tx, internal_event_rx) = mpsc::unbounded_channel();
		Self {
			event_tx,
			internal_event_tx,
			internal_event_rx,
			rx
		}
	}

	async fn handle_cmd(&mut self, cmd: InternalCommand) {
		match cmd {
			InternalCommand::Bind { addr } => todo!(),
			InternalCommand::Connect { addr } => todo!(),
			InternalCommand::PeerCmd { addr, cmd } => todo!(),
		}
	}

	async fn handle_interal_event(&mut self, event: InternalEvent) {
		match event {
			InternalEvent::PeerConnected { addr, tx } => {
				self.event_tx.send(PupynetEvent::PeerConnected { addr, tx }).unwrap();
			},
			InternalEvent::PeerDisconnected { addr } => {
				self.event_tx.send(PupynetEvent::PeerDisconnected { addr }).unwrap();
			},
			InternalEvent::PeerCmd { addr, cmd } => {
				match cmd {
						PeerCmd::ReadFile { node_id, path, offset, length } => todo!(),
						PeerCmd::WriteFile { node_id, path, offset, data } => todo!(),
						PeerCmd::RemoveFile { node_id, path } => todo!(),
						PeerCmd::CreateFolder { node_id, path } => todo!(),
						PeerCmd::RenameFolder { node_id, path, new_name } => todo!(),
						PeerCmd::RemoveFolder { node_id, path } => todo!(),
						PeerCmd::ListFolderContents { node_id, path, offset, length, recursive } => todo!(),
						PeerCmd::Introduce(introduce) => {
							
						},
						PeerCmd::Hello => todo!(),
					}
			}
		}
	}

	pub async fn run(mut self) {
		loop {
			tokio::select! {
				cmd = self.rx.recv() => {
					match cmd {
						Some(cmd) => {
							self.handle_cmd(cmd).await;
						},
						None => {
							log::error!("command channel closed");
							break;
						}
					}
				}
				event = self.internal_event_rx.recv() => {
					match event {
						Some(event) => {
							self.handle_interal_event(event).await;
						},
						None => {
							log::error!("internal event channel closed");
							break;
						}
					}
				}
			}
		}
	}
}