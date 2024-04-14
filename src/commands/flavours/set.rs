use crate::{
    commands::command::Command,
    configuration::configuration::Settings,
    markdown::flavour::{Flavour, FlavourSettings},
};

use super::flavour::FlavourCommand;

pub struct FlavourSetCommand;

impl Command for FlavourSetCommand {
    fn execute(settings: Settings, _arguments: Option<Vec<String>>) {
        let callback = |settings: Settings, flavour: Flavour| {
            FlavourSettings::set_flavour(
                &settings.application.configuration_dir,
                flavour.to_owned(),
            );
        };

        FlavourCommand::handle_flavour_selection(
            settings, "set", None, callback,
        );
    }

    fn help() {
        println!("\t\t--set: Set a flavour for the markdown file");
    }
}
