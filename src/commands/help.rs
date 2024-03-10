use crate::commands::command::Command;
use crate::configuration::Settings;

use super::{flavour::flavour_base::FlavourCommand, VersionCommand};

pub struct HelpCommand;

impl Command for HelpCommand {
    fn execute(_settings: Settings, _arguments: Option<Vec<String>>) {}

    fn help() {
        println!(
            "Usage: smd --input <input_file> --output <output_type> --specific <?specific_type>"
        );
        println!("\t--init: Initializes the tool. Is required to be run when using smd for the first time\n");
        println!("\t--input: provide a markdown file or a path to a markdown file\n");
        println!("\t--output: specify a filetype for the output, a file with the same as the input file name will be generated. Available types are:");
        println!("\t\tpdf");
        println!("\t\thtml");
        println!("\t\timg\n");
        println!("\t--specific: Is ONLY used when you want to convert markdown into an image. You need to provide what type of image you want. Available types are:");
        println!("\t\tpng");
        println!("\t\twebp");
        println!("\t\tjpeg\n");
        FlavourCommand::help();
        VersionCommand::help();
        println!(
            "If more information is needed consult the README.md on https://github.com/Flokkq/smd"
        );
    }
}
