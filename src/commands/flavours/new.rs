use std::str::FromStr;

use crate::{
    commands::command::Command,
    configuration::configuration::Settings,
    error::invalid_argument_message,
    markdown::flavour::{Flavour, FlavourSettings},
};

use super::flavour::FlavourCommand;

pub struct FlavourNewCommand;

impl Command for FlavourNewCommand {
    fn execute(settings: Settings, arguments: Option<Vec<String>>) {
        let callback = |settings: Settings, selected_flavour: Flavour| {
            let arguments = arguments.unwrap_or_else(|| vec![]);

            if arguments.len() != 4 {
                invalid_argument_message();
            }

            let flavour =
                Flavour::from_str(&arguments.get(3).unwrap().to_string())
                    .unwrap();
            FlavourSettings::new_flavour(
                &settings.application.configuration_dir,
                flavour,
                selected_flavour.to_owned(),
            )
        };

        FlavourCommand::handle_flavour_selection(
            settings,
            "use as a template",
            None,
            callback,
        );
    }

    fn help() {
        println!("\t\t--new: Specify a name for your theme");
        println!("\t\t\tA yaml file wii appear in your current directory. You can build a new flavohr from this file.");
        println!("\t\t\tYou will also be able to choose from templates of your previous and standad flavours");
    }
}
