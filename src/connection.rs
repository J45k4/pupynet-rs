use tokio::io::AsyncRead;
use tokio::io::AsyncWrite;
use tokio::io::AsyncReadExt;

use crate::multiplex::Multiplexer;
use crate::multiplex::MultiplexerEvent;
use crate::types::Context;

pub struct Connection<T: AsyncRead + AsyncWrite> {
	conn: T,
	ctx: Context,
	multiplexer: Multiplexer
}

impl<T: AsyncRead + AsyncWrite + std::marker::Unpin> Connection<T> {
	pub fn new(conn: T, ctx: Context) -> Connection<T> {
		Connection {
			conn,
			ctx,
			multiplexer: Multiplexer::new()
		}
	}

	pub async fn run(mut self) {
		let mut buffer = [0u8; 1024];
		loop {
			let n = self.conn.read(&mut buffer).await.unwrap();
			if n == 0 {
				break;
			}
			self.multiplexer.handle_data(&buffer[0..n], |event| match event {
				MultiplexerEvent::StreamStarted { stream_id, length } => {
					
				},
				MultiplexerEvent::DataPointer { stream_id, data } => {},
				MultiplexerEvent::StreamEnded { stream_id } => {},
				MultiplexerEvent::Error(_) => {},
			});
		}
	}
}