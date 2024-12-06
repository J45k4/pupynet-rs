use args::Args;
use args::Command;
use pupynet_core::Pupynet;
use clap::Parser;

mod args;

#[tokio::main]
async fn main() {
	let args = Args::parse();
}
