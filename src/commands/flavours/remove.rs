use std::io::{stdout, Write};

use crate::{
    commands::command::Command,
    configuration::configuration::Settings,
    markdown::flavour::{Flavour, FlavourSettings},
};

use super::flavour::FlavourCommand;

pub struct FlavourRemoveCommand;

impl Command for FlavourRemoveCommand {
    fn execute(settings: Settings, _arguments: Option<Vec<String>>) {
        let info = "Note: The currently selected flavor will remain active until a new selection is made, even if it is removed.";
        let callback = |settings: Settings, flavour: Flavour| {
            FlavourSettings::remove_flavour(
                &settings.application.configuration_dir,
                flavour.to_owned(),
            );
        };

        FlavourCommand::handle_flavour_selection(
            settings,
            "remove",
            Some(info),
            callback,
        );
    }

    fn help() {
        println!("\t\t--remove: Remove an added flavour");
    }
}
