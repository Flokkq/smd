use clap::{
	ArgAction,
	Args,
	Parser,
	Subcommand,
	ValueEnum,
};
use std::path::PathBuf;

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum OutputFileFormat {
	Pdf,
	Html,
}

/// Command-line arguments to parse.
#[derive(Debug, Parser)]
#[command(
    version,
    rename_all_env = "screaming-snake",
    help_template = "\
{before-help}{name} {version} by {author-with-newline}
{usage-heading}
  {usage}

{all-args}{after-help}
",
    override_usage = "smd [OPTIONS] [FLAGS]",
    author = clap::crate_authors!("\n"),
    next_help_heading = Some("FLAGS"),
    disable_help_flag = true,
    disable_version_flag = true,
    disable_help_subcommand = true,
)]
pub struct Cli {
	#[command(subcommand)]
	pub commands: Commands,

	/// Prints help information.
	#[arg(
        short,
        long,
        action = ArgAction::Help,
        global = true,
        help = "Prints help information",
        help_heading = Some("FLAGS"),
    )]
	pub help: Option<bool>,

	/// Prints version information.
	#[arg(
        short = 'V',
        long,
        action = ArgAction::Help,
        global = true,
        help = "Prints version information",
        help_heading = Some("FLAGS"),
    )]
	pub version: Option<bool>,

	/// Increases the logging verbosity.
	#[arg(
        short,
        long,
        action = ArgAction::Count,
        alias = "debug",
        help_heading = Some("FLAGS"),
    )]
	pub verbose: u8,
}

#[derive(Debug, Subcommand, PartialEq)]
pub enum Commands {
	/// Parse and convert Markdown files.
	Parse(ParseArgs),

	/// Writes the default configuration to the systems configuration
	/// diretctory
	Initialize,
}

#[derive(Debug, Args, PartialEq)]
pub struct ParseArgs {
	/// Sets the markdown file to convert.
	#[arg(
        short,
        long,
        value_name = "PATH",
        value_parser = Cli::parse_dir,
        help_heading = Some("OPTIONS"),
        required=true,
    )]
	pub input: PathBuf,

	/// Sets file format for the output file.
	#[arg(
        long,
        short,
        value_enum,
        value_name = "OUTPUT_FILE_FORMAT",
        default_value_t = OutputFileFormat::Pdf,
        help_heading = Some("OPTIONS"),
    )]
	pub output: OutputFileFormat,
}

impl Cli {
	/// Custom string parser for directories.
	///
	/// Expands the tilde (`~`) character in the beginning of the
	/// input string into contents of the path returned by [`home_dir`].
	///
	/// [`home_dir`]: dirs::home_dir
	fn parse_dir(dir: &str) -> Result<PathBuf, String> {
		Ok(PathBuf::from(shellexpand::tilde(dir).to_string())
			.canonicalize()
			.map_err(|e| format!("Failed to canonicalize file path: {}", e))?)
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use clap::CommandFactory;

	#[test]
	fn verify_cli() {
		Cli::command().debug_assert();
	}

	#[test]
	fn path_tilde_expansion() {
		let home_dir =
			dirs::home_dir().expect("cannot retrieve home directory");
		let dir = Cli::parse_dir("~/").expect("cannot expand tilde");
		assert_eq!(home_dir, dir);
	}
}
