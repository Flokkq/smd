use args::{
	Cli,
	Commands,
};
use smd_core::error::Result;

pub mod args;

/// Runs `smd`.
pub fn run(cli: Cli) -> Result<()> {
	match cli.commands {
		Commands::Parse(args) => {
			let _ = smd_core::fs::read_to_string(&args.input)?;
		}
	}
	return Ok(());
}
