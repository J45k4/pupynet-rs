use tokio::sync::broadcast;
use tokio::sync::mpsc;
use crate::protocol::Introduce;
use crate::protocol::PeerCmd;
use crate::types::InternalCommand;
use crate::types::InternalEvent;
use crate::types::State;
use crate::udp::bind_default;
use crate::PupynetEvent;

pub struct Worker {
	event_tx: broadcast::Sender<PupynetEvent>,
	internal_event_tx: mpsc::UnboundedSender<InternalEvent>,
	internal_event_rx: mpsc::UnboundedReceiver<InternalEvent>,
	rx: mpsc::UnboundedReceiver<InternalCommand>,
	udp_socket: std::sync::Arc<tokio::net::UdpSocket>,
	state: State
}

impl Worker {
	pub async fn new(rx: mpsc::UnboundedReceiver<InternalCommand>, event_tx: broadcast::Sender<PupynetEvent>) -> Self {
		let (internal_event_tx, internal_event_rx) = mpsc::unbounded_channel();

		let udp_socket = bind_default(internal_event_tx.clone()).await;

		Self {
			event_tx,
			internal_event_tx,
			internal_event_rx,
			rx,
			udp_socket,
			state: State::default()
		}
	}

	async fn send(&self, addr: &str, cmd: PeerCmd) {
		if addr.starts_with("udp://") {
			let addr = addr.trim_start_matches("udp://");
			self.udp_socket.send_to(&cmd.serialize(), &addr).await.unwrap();
			return;
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
				self.event_tx.send(PupynetEvent::PeerConnected { addr }).unwrap();
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
							if introduce.id == self.state.me.id {
								log::info!("it is me");
								return;
							}
	
							let peer = self.state.peers.entry(introduce.id.clone()).or_default();
							peer.id = introduce.id;
							peer.name = introduce.name;
							if !peer.introduced {
								peer.introduced = true;
								let cmd = PeerCmd::Introduce(Introduce {
									id: self.state.me.id.clone(),
									name: self.state.me.name.clone(),
									owner: self.state.me.owner.clone().unwrap_or_default(),
								});
								self.send(&addr, cmd).await;
							}
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