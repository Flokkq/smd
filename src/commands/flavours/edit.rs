use std::{
    io::{stdout, Write},
    str::FromStr,
};

use crate::{
    commands::command::Command,
    configuration::configuration::Settings,
    markdown::flavour::{Flavour, FlavourSettings},
};

use super::flavour::FlavourCommand;

pub struct FlavourEditCommand;

impl Command for FlavourEditCommand {
    fn execute(settings: Settings, _arguments: Option<Vec<String>>) {
        let callback = |settings: Settings, flavour: Flavour| {
            FlavourSettings::edit_flavour(
                &settings.application.configuration_dir,
                flavour.to_owned(),
            );
        };

        FlavourCommand::handle_flavour_selection(
            settings, "edit", None, callback,
        );
    }

    fn help() {
        println!("\t\t--edit: Provide the name of the flavour you want to edit. If ran without arguments then a list with all available flavours will be shown");
        println!(
            "\t\t\tRun smd --flavour --update if you made changes to your file"
        );
    }
}
