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

const HTML_FILE_ENDING: &str = "html";

/// Runs `smd`.
pub fn run(cli: Cli) -> Result<()> {
	match cli.commands {
		Commands::Parse(args) => {
			let content = fs::read_to_string(&args.input)?;

			info!("Transpiling markdown");
			let result = gfm::Parser::render(&content);

			let mut out_path = args.input.clone();

			if args.output.eq(&args::OutputFileFormat::Html) {
				out_path.set_extension(HTML_FILE_ENDING);
				fs::write_to_file(&out_path, &result)?;

				return Ok(());
			}
		}
	}
	return Ok(());
}
