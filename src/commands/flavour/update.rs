use crate::commands::Command;
use crate::configuration::Settings;

pub struct FlavourUpdateCommand;

impl Command for FlavourUpdateCommand {
    fn execute(settings: Settings, arguments: Option<Vec<String>>) {
        todo!()
    }

    fn help() {
        println!("\t\t--update: Update all added css files\n");
    }
}
