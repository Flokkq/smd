use clap::Parser;
use smd::args::Cli;
use smd::logger;
use smd_core::error::Result;
use std::{
	env,
	process,
};

fn main() -> Result<()> {
	let args = Cli::parse();
	if args.verbose == 1 {
		env::set_var("RUST_LOG", "debug");
	} else if args.verbose > 1 {
		env::set_var("RUST_LOG", "trace");
	} else if env::var_os("RUST_LOG").is_none() {
		env::set_var("RUST_LOG", "info");
	}
	logger::init()?;

	let exit_code = match smd::run(args) {
		Ok(()) => 0,
		Err(e) => {
			log::error!("{}", e);
			1
		}
	};

	process::exit(exit_code);
}
