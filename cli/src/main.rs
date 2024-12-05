use app::App;
use args::Args;
use args::Command;
use pupynet_core::Pupynet;
use clap::Parser;

mod args;
mod app;

#[tokio::main]
async fn main() {
	let args = Args::parse();

    let pupynet = Pupynet::new();
	
	let cmd = match args.cmd {
		Some(cmd) => cmd,
		None => {
			println!("no command specified");
			return;
		}
	};

	match cmd {
		Command::Gui => {
			iced::application("Pupynet", App::update, App::view)
				.subscription(f)
				.
				.run_with(App::new)
				.unwrap();
		}
		Command::Copy { src, dest } => {
			println!("copy {} to {}", src, dest);
		}
	}
}
