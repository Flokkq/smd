use std::{
    io::{stdout, Write},
    path::PathBuf,
    str::FromStr,
};

use crate::{
    commands::command::Command,
    configuration::configuration::Settings,
    error::invalid_argument_message,
    markdown::flavour::{Flavour, FlavourSettings},
};

use super::flavour::FlavourCommand;

pub struct FlavourUpdateCommand;

impl Command for FlavourUpdateCommand {
    fn execute(settings: Settings, arguments: Option<Vec<String>>) {
        let callback = |settings: Settings, flavour: Flavour| {
            let arguments = arguments.unwrap_or_else(|| vec![]);

            if arguments.len() != 4 {
                invalid_argument_message();
            }

            update_flavour(
                flavour.to_owned(),
                &settings.application.configuration_dir,
                arguments,
            );
        };

        FlavourCommand::handle_flavour_selection(
            settings, "update", None, callback,
        );
    }

    fn help() {
        println!("\t\t--update: Select a flavour to update after running smd --flavour --edit.");
        println!("\t\t\tNote that a copy of the file will be made.")
    }
}

fn update_flavour(
    flavour: Flavour,
    configuration_dir: &PathBuf,
    arguments: Vec<String>,
) {
    let arg = arguments.get(3).unwrap();
    let arg = if arg.ends_with(".yaml") {
        arg.to_string()
    } else {
        format!("{}.yaml", arg)
    };

    let path_to_flavour = PathBuf::from_str(&arg).unwrap();
    let flavour_to_update = Flavour::from_str(
        path_to_flavour.file_stem().unwrap().to_str().unwrap(),
    )
    .unwrap();

    let flavour_settings =
        FlavourSettings::new(flavour_to_update, &configuration_dir);

    if flavour_settings.flavour.eq(&flavour) {
        FlavourSettings::update_flavour(
            &configuration_dir,
            flavour_settings,
            path_to_flavour,
            None,
        )
    } else {
        FlavourSettings::update_flavour(
            &configuration_dir,
            flavour_settings,
            path_to_flavour,
            Some(flavour),
        )
    }
}
