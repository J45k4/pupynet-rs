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