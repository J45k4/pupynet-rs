
use hyper::server::conn::http1;
use hyper::service::service_fn;
use tokio::io::AsyncReadExt;
use tokio::net::TcpListener;
use tokio::net::TcpStream;

use crate::connection::Connection;
use crate::multiplex::Multiplexer;
use crate::types::Context;


pub async fn bind(addr: &str, ctx: Context) {
	let listener = TcpListener::bind(addr).await.expect("Failed to bind");
	while let Ok((stream, _)) = listener.accept().await {
		let ctx = ctx.clone();
		tokio::spawn(async move {
			Connection::new(stream, ctx).run().await;
		});
	}
}