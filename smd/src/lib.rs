use args::{
	Cli,
	Commands,
};
use log::info;
use smd_core::{
	config::Config,
	error::{
		Error,
		Result,
	},
	fs,
	gfm,
	DEFAULT_CONFIG,
};

pub mod args;
pub mod logger;

const HTML_FILE_ENDING: &str = "html";

/// Runs `smd`.
pub fn run(cli: Cli) -> Result<()> {
	if Commands::Initialize == cli.commands {
		return initialize();
	}

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

		_ => unreachable!(),
	}
	return Ok(());
}

fn initialize() -> Result<()> {
	let config = Config::default();
	let toml = toml::to_string_pretty(&config)?;

	let path = dirs::config_dir().unwrap_or_default().join(DEFAULT_CONFIG);

	if path.exists() {
		return Err(Error::ConfigAlreadyExistsError(
			path.to_string_lossy().to_string(),
		));
	}

	if let Some(parent) = path.parent() {
		std::fs::create_dir_all(parent)?;
	}
	fs::write_to_file(&path, &toml)?;

	Ok(())
}
