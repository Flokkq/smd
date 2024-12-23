use args::{
	Cli,
	Commands,
};
use log::info;
use smd_core::{
	config::Config,
	convert::{
		self,
		PDFConverter,
	},
	error::Result,
	fs,
};

pub mod args;
pub mod logger;

const HTML_FILE_ENDING: &str = "html";
const PDF_FILE_ENDING: &str = "pdf";

/// Runs `smd`.
pub fn run(cli: Cli) -> Result<()> {
	if Commands::Initialize == cli.commands {
		return Config::initialize();
	}

	// TODO: use config in parser & convert trait
	let _config = Config::load_config()?;

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

			let converted = match args.output {
				args::OutputFileFormat::Pdf => {
					out_path.set_extension(PDF_FILE_ENDING);
					convert::convert_html::<PDFConverter>(&content)?
				}
				_ => unreachable!(),
			};

			fs::write_bytes(&out_path, &converted)?;
		}

		_ => unreachable!(),
	}
	Ok(())
}
