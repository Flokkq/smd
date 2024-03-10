use crate::commands::Command;
use crate::configuration::Settings;

pub struct VersionCommand;

impl Command for VersionCommand {
    fn execute(settings: Settings, arguments: Option<Vec<String>>) {
        todo!()
    }
    fn help() {
        println!("\t--version: prints the current version");
    }
}
