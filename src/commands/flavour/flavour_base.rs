use crate::commands::command::Command;
use crate::commands::flavour::add::FlavourAddCommand;
use crate::commands::flavour::remove::FlavourRemoveCommand;
use crate::commands::flavour::set::FlavourSetCommand;
use crate::commands::flavour::update::FlavourUpdateCommand;
use crate::configuration::Settings;
use crate::utils::invalid_argument_message;

pub struct FlavourCommand;

impl Command for FlavourCommand {
    fn execute(settings: Settings, arguments: Option<Vec<String>>) {
        match arguments {
            Some(arguments) => match arguments.get(2).unwrap().as_str() {
                "--set" => FlavourSetCommand::execute(settings, None),
                "--add" => FlavourAddCommand::execute(settings, None),
                "--remove" => FlavourRemoveCommand::execute(settings, None),
                "--update" => FlavourUpdateCommand::execute(settings, None),
                _ => invalid_argument_message(),
            },
            None => (),
        }
    }
    fn help() {
        println!("\t--flavour: Add a custom flavour or specify a flavour (theme) for the markdown files.");
        FlavourSetCommand::help();
        FlavourAddCommand::help();
        FlavourRemoveCommand::help();
        FlavourUpdateCommand::help();
    }
}
