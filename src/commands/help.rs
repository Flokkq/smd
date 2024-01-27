use crate::commands::command::Command;
use crate::configuration::Settings;

pub struct HelpCommand;

impl Command for HelpCommand {
    fn execute(_settings: Settings, _arguments: Option<Vec<String>>) {
        println!("Usage: smd --input <input_file> --output <output_type> --specific <?specific_type>");
        println!("\t--input: provide a markdown file or a path to a markdown file\n");
        println!("\t--output: specify a filetype for the output, a file with the same as the input file name will be generated. Available types are:");
        println!("\t\tpdf");
        println!("\t\thtml");
        println!("\t\timg\n");
        println!("\t--specific: Is ONLY used when you want to convert markdown into an image. You need to provide what type of image you want. Available types are:");
        println!("\t\tpng");
        println!("\t\twebp");
        println!("\t\tjpeg\n");
        println!("\t--init: Initializes the tool. Is required to be run when using smd for the first time\n");
        println!("\t--flavour: Add a custom flavour or specify a flavour (theme) for the markdown files.");
        println!("\t\t--set: Set a flavour for the markdown file");
        println!("\t\t--add: Provide a path to a css file. Note that a copy of the file will be made. Run smd --flavour --update if you made changes to your file");
        println!("\t\t--remove: Remove an added css file");
        println!("\t\t--update: Update all added css files\n");
        println!("\t--version: prints the current version");
    }
}