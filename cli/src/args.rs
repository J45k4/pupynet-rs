use clap::Parser;
use clap::Subcommand;


#[derive(Debug, Parser)]
#[clap(name = "puppydrive")]
pub struct Args {
    #[clap(long)]
    pub peer: Vec<String>,
    #[clap(long)]
    pub bind: Vec<String>,
    #[clap(long, default_value = "127.0.0.1:8832")]
    pub ui_bind: String,
	#[clap(subcommand)]
	pub cmd: Option<Command>,
}

#[derive(Debug, Parser)]
pub enum Command {
	Copy { src: String, dest: String },
	Update,
	Verify { bin: String, sig: String },
}
