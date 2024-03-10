use crate::commands::Command;
use crate::configuration::Settings;

struct FlavourEditCommand;

impl Command for FlavourEditCommand {
    fn execute(settings: Settings, arguments: Option<Vec<String>>) {}

    fn help() {
        println!("\t\t--edit: Provide the name of the flavour you want to edit. If ran without arguments then a list with all available flavours will be shown");
        println!("\t\tRun smd --flavour --update if you made changes to your file");
    }
}
