use super::command::Command;
use crate::configuration::configuration::Settings;

pub struct VersionCommand;

impl Command for VersionCommand {
    fn execute(_settings: Settings, _arguments: Option<Vec<String>>) {
        VersionCommand::help();
    }

    fn help() {
        println!("\t--version: prints the current version");
    }
}
