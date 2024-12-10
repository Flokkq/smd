use args::{
	Cli,
	Commands,
};
use log::info;
use smd_core::{
	error::Result,
	fs,
	gfm,
};

pub mod args;
pub mod logger;

/// Runs `smd`.
pub fn run(cli: Cli) -> Result<()> {
	match cli.commands {
		Commands::Parse(args) => {
			let content = fs::read_to_string(&args.input)?;

			info!("Transpiling markdown");
			let _ = gfm::Parser::render(&content);
		}
	}
	return Ok(());
}
