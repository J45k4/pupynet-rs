use std::collections::HashMap;

use tokio::sync::mpsc;

use crate::protocol::PeerCmd;

pub enum PeerConnCmd {
	Close,
	Send(Vec<u8>)
}

pub enum InternalEvent {
	PeerConnected {
		addr: String,
		tx: mpsc::UnboundedSender<PeerConnCmd>
	},
	PeerDisconnected {
		addr: String
	},
	PeerCmd {
		addr: String,
		cmd: PeerCmd
	}
}

pub enum InternalCommand {
	Bind {
		addr: String
	},
	Connect {
		addr: String
	},
	PeerCmd {
		addr: String,
		cmd: PeerCmd
	}
}

#[derive(Debug, Default)]
pub struct Peer {
	pub id: String,
	pub name: String,
	pub owner: Option<String>,
	pub introduced: bool,
}

#[derive(Debug, Default)]
pub struct State {
	pub me: Peer,
	pub peers: HashMap<String, Peer>
}

impl State {

}

pub struct Context {

}

impl Clone for Context {
	fn clone(&self) -> Self {
		Context {}
	}
}