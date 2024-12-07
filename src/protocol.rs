pub const INTRODUCE_CMD: u16 = 1;
pub const CMD_WRITE_FILE: u16 = 2;
pub const CMD_READ_FILE: u16 = 3;
pub const CMD_REMOVE: u16 = 4;
pub const CMD_CREATE_FOLDER: u16 = 5;
pub const CMD_MOVE: u16 = 6;
pub const CMD_LIST_FOLDER_CONTENTS: u16 = 7;
pub const CMD_PEER_CONNECTED: u16 = 8;
pub const CMD_PEER_DISCONNECTED: u16 = 9;
pub const CMD_EXECUTE: u16 = 10;
pub const CMD_FORGET_PEER: u16 = 11;

pub const STREAM_START: u8 = 0x01;
pub const STREAM_END: u8 = 0x02;
pub const STREAM_CONTINUE: u8 = 0x03;
pub const STREAM_PAUSE: u8 = 0x04;
pub const STREAM_PULL: u8 = 0x05;
pub const STREAM_DIED: u8 = 0x06;

pub const SUCCES: u8 = 0x00;
pub const ERR_REMOVE_FOLDER_RECURSIVE_NOT_ENABLED: u8 = 0x01;

#[derive(Debug)]
pub struct Introduce {
	pub id: String,
	pub name: String,
	pub owner: String,
}

#[derive(Debug)]
pub enum PeerCmd {
    ReadFile {
        node_id: String,
        path: String,
        offset: u64,
        length: u64,
    },
    WriteFile {
        node_id: String,
        path: String,
        offset: u64,
        data: Vec<u8>,
    },
    RemoveFile {
        node_id: String,
        path: String,
    },
    CreateFolder {
        node_id: String,
        path: String,
    },
    RenameFolder {
        node_id: String,
        path: String,
        new_name: String,
    },
    RemoveFolder {
        node_id: String,
        path: String,
    },
    ListFolderContents {
        node_id: String,
        path: String,
        offset: u64,
        length: u64,
        recursive: bool,
    },
	Introduce(Introduce),
	Hello
}

impl PeerCmd {
	pub fn serialize(&self) -> Vec<u8> {
		match self {
			PeerCmd::Introduce(args) => {
				let id_len_bytes = (args.id.len() as u16).to_le_bytes();
				let id_bytes = args.id.as_bytes();
				let name_len_bytes = (args.name.len() as u16).to_le_bytes();
				let name_bytes = args.name.as_bytes();
				let owner_len_bytes = (args.owner.len() as u16).to_le_bytes();
				let owner_bytes = args.owner.as_bytes();
				let payload_size = 2 + id_bytes.len() + 2 + name_bytes.len() + 2 + owner_bytes.len();
				let mut res = Vec::with_capacity(6 + payload_size);
				res.extend_from_slice(&INTRODUCE_CMD.to_le_bytes());
				res.extend_from_slice(&(payload_size as u32).to_le_bytes());
				res.extend_from_slice(&id_len_bytes);
				res.extend_from_slice(id_bytes);
				res.extend_from_slice(&name_len_bytes);
				res.extend_from_slice(name_bytes);
				res.extend_from_slice(&owner_len_bytes);
				res.extend_from_slice(owner_bytes);
				res
			}
			_ => todo!()
		}
	}
}

struct ByteEater<'a> {
	buffer: &'a [u8]
}

impl<'a> ByteEater<'a> {
	pub fn new(buffer: &'a [u8]) -> Self {
		Self {
			buffer
		}
	}
	
	pub fn get_u16(&mut self) -> u16 {
		u16::from_le_bytes(self.buffer[0..2].try_into().unwrap())
	}

	pub fn get_string(&mut self) -> String {
		let len = self.get_u16();
		let s = self.buffer[2..len as usize + 2].to_vec();
		self.buffer = &self.buffer[len as usize + 2..];
		String::from_utf8(s).unwrap()
	}
}