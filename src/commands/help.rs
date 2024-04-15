use crate::{
    commands::{flavours::flavour::FlavourCommand, version::VersionCommand},
    configuration::configuration::Settings,
};

use super::command::Command;

pub struct HelpCommand;

impl Command for HelpCommand {
    fn execute(_settings: Settings, _arguments: Option<Vec<String>>) {
        HelpCommand::help();
        FlavourCommand::help();
        VersionCommand::help();
        println!(
            "If more information is needed consult the README.md on https://github.com/Flokkq/smd"
        );
    }

    fn help() {
        println!(
            "Usage: smd --input <input_file> --output <output_type> [--specific <specific_type>] [--apply-flavour <flavour_name>]"
        );
        println!(
            "\t--input: provide a markdown file or a path to a markdown file\n"
        );
        println!("\t--output: specify a filetype for the output, a file with the same as the input file name will be generated. Available types are:");
        println!("\t\tpdf");
        println!("\t\thtml");
        println!("\t\timg\n");
        println!("\t--specific: Is ONLY used when you want to convert markdown into an image. You need to provide what type of image you want. Available types are:");
        println!("\t\tpng");
        println!("\t\twebp");
        println!("\t\tjpeg\n");
        println!("\t--apply-flavour: Is ONLY used when you want to apply a specific flavour for a single conversion\n");
    }
}
