use fastwebsockets::upgrade;
use fastwebsockets::FragmentCollector;
use fastwebsockets::OpCode;
use fastwebsockets::WebSocketError;
use http_body_util::Empty;
use hyper::body::Bytes;
use hyper::body::Incoming;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::Request;
use hyper::Response;
use tokio::net::TcpListener;
use tokio::net::TcpStream;

// use crate::stream::Transport;

// struct WsTransport<T> {
// 	ws: FragmentCollector<T>
// }

// impl Transport for WsTransport {

// }

async fn handle_client(fut: upgrade::UpgradeFut) -> Result<(), WebSocketError> {

	let mut ws: FragmentCollector<hyper_util::rt::TokioIo<hyper::upgrade::Upgraded>> = fastwebsockets::FragmentCollector::new(fut.await?);
	loop {
	  let frame = ws.read_frame().await?;
	  match frame.opcode {
		OpCode::Close => break,
		OpCode::Text | OpCode::Binary => {
		// 	ws.write_frame(frame)
		// 	frame.payload.as_bytes()
		//   ws.write_frame(frame).await?;
		}
		_ => {}
	  }
	}
  
	Ok(())
  }

async fn server_upgrade(
	mut req: Request<Incoming>,
  ) -> Result<Response<Empty<Bytes>>, WebSocketError> {
	if upgrade::is_upgrade_request(&req) {
		let (res, fut) = upgrade::upgrade(&mut req)?;

		return Ok(res);
	}

	
	let (response, fut) = upgrade::upgrade(&mut req)?;
  
	tokio::task::spawn(async move {
	  if let Err(e) = tokio::task::unconstrained(handle_client(fut)).await {
		eprintln!("Error in websocket connection: {}", e);
	  }
	});
  
	Ok(response)
  }

async fn handle_stream(stream: TcpStream) {
	let io = hyper_util::rt::TokioIo::new(stream);
	let conn_fut = http1::Builder::new()
          .serve_connection(io, service_fn(server_upgrade))
          .with_upgrades();
	if let Err(e) = conn_fut.await {
		println!("An error occurred: {:?}", e);
	}
}

pub async fn bind(addr: &str) {
	let listener = TcpListener::bind(addr).await.expect("Failed to bind");
	while let Ok((stream, _)) = listener.accept().await {
		tokio::spawn(async move {
			handle_stream(stream).await;
		});
	}
}