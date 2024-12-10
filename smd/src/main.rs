use clap::Parser;
use smd::args::Cli;
use smd_core::error::Result;
use std::process;

fn main() -> Result<()> {
	let args = Cli::parse();

	let exit_code = match smd::run(args) {
		Ok(()) => 0,
		Err(e) => {
			log::error!("{}", e);
			1
		}
	};

	process::exit(exit_code);
}
