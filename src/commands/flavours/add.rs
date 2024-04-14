use std::path::PathBuf;
use std::process::exit;
use std::str::FromStr;

use crate::commands::command::Command;
use crate::configuration::configuration::Settings;
use crate::error::invalid_argument_message;
use crate::markdown::flavour::{Flavour, FlavourSettings};

pub struct FlavourAddCommand;

impl Command for FlavourAddCommand {
    fn execute(settings: Settings, arguments: Option<Vec<String>>) {
        let arguments = arguments.unwrap_or_else(|| vec![]);

        if arguments.len() != 4 {
            invalid_argument_message();
        }

        let flavours = FlavourSettings::load_flavours(
            &settings.application.configuration_dir,
        );

        let arg = arguments.get(3).unwrap();
        let arg = if arg.ends_with(".yaml") {
            arg.to_string()
        } else {
            format!("{}.yaml", arg)
        };

        let path_to_flavour = PathBuf::from_str(&arg).unwrap();
        let flavour_to_add = Flavour::from_str(
            path_to_flavour.file_stem().unwrap().to_str().unwrap(),
        )
        .unwrap();

        if flavours.contains(&flavour_to_add) {
            log::warn!("Flavour already exists\nIf you want to update an already exisiting flavour run smd --help for furhter information");
            exit(1);
        }

        let flavour_settings = FlavourSettings::new(
            flavour_to_add,
            &settings.application.configuration_dir,
        );

        FlavourSettings::add_flavour(
            &settings.application.configuration_dir,
            flavour_settings,
            path_to_flavour,
        );
    }

    fn help() {
        println!("\t\t--add: Provide a path to a yaml file. Note that a copy of the file will be made. ");
        println!(
            "\t\t\tRun smd --flavour --update if you made changes to your file"
        )
    }
}
