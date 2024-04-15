use std::io::{stdout, Write};

use crate::{
    commands::{
        command::Command,
        flavours::{
            add::FlavourAddCommand, current::FlavourCurrentCommand,
            edit::FlavourEditCommand, list::FlavourListCommand,
            remove::FlavourRemoveCommand, set::FlavourSetCommand,
            update::FlavourUpdateCommand,
        },
    },
    configuration::configuration::Settings,
    error::invalid_argument_message,
    markdown::flavour::{Flavour, FlavourSettings},
};

use super::new::FlavourNewCommand;

pub struct FlavourCommand;

impl Command for FlavourCommand {
    fn execute(settings: Settings, arguments: Option<Vec<String>>) {
        match arguments {
            Some(arguments) => match arguments.get(2).unwrap().as_str() {
                "--set" => FlavourSetCommand::execute(settings, None),
                "--new" => {
                    FlavourNewCommand::execute(settings, Some(arguments))
                }
                "--add" => {
                    FlavourAddCommand::execute(settings, Some(arguments))
                }
                "--edit" => {
                    FlavourEditCommand::execute(settings, Some(arguments))
                }
                "--remove" => FlavourRemoveCommand::execute(settings, None),
                "--update" => {
                    FlavourUpdateCommand::execute(settings, Some(arguments))
                }
                "--list" => FlavourListCommand::execute(settings, None),
                "--current" => FlavourCurrentCommand::execute(settings, None),
                "" => FlavourCommand::help(),
                _ => invalid_argument_message(),
            },
            None => (),
        }
    }

    fn help() {
        println!("\t--flavour: Add a custom flavour or specify a flavour (theme) for the markdown files.");
        FlavourSetCommand::help();
        FlavourNewCommand::help();
        FlavourAddCommand::help();
        FlavourRemoveCommand::help();
        FlavourEditCommand::help();
        FlavourUpdateCommand::help();
        FlavourListCommand::help();
        FlavourCurrentCommand::help();

        println!("");
    }
}

impl FlavourCommand {
    pub fn handle_flavour_selection<F>(
        settings: Settings,
        action: &str,
        extra_information: Option<&str>,
        callback: F,
    ) where
        F: FnOnce(Settings, Flavour),
    {
        let flavours = FlavourSettings::load_flavours(
            &settings.application.configuration_dir,
        );

        println!("Which flavour would you like to {}?", action);
        if let Some(extra) = extra_information {
            println!("{}", extra);
        }

        for (i, flavour) in flavours.iter().enumerate() {
            println!("\t{} > {}", i + 1, flavour.to_string().to_lowercase());
        }

        loop {
            print!("flavour: ");
            stdout().flush().unwrap();

            let mut flavour = String::new();
            std::io::stdin()
                .read_line(&mut flavour)
                .expect("Failed reading input");

            let selected_flavour = match flavour.trim().parse::<usize>() {
                Ok(num) => num,
                Err(_) => {
                    log::warn!("Invalid input! Please provide a valid number.");
                    continue;
                }
            };

            if selected_flavour > 0 && selected_flavour <= flavours.len() {
                if let Some(flavour) = flavours.get(selected_flavour - 1) {
                    callback(settings, flavour.to_owned());
                }

                break;
            }
            log::warn!("Invalid input! Please provide a valid number within the range.");
        }
    }
}
