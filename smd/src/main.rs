use std::process;

use clap::Parser;
use smd::args::Cli;

fn main() {
	let _ = Cli::parse();

	process::exit(0);
}
