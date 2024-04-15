use crate::{
    commands::{command::Command, flavours::current::FlavourCurrentCommand},
    markdown::flavour::{Flavour, FlavourSettings},
};

pub struct FlavourListCommand;

impl Command for FlavourListCommand {
    fn execute(
        settings: crate::configuration::configuration::Settings,
        _arguments: Option<Vec<String>>,
    ) {
        let flavours = FlavourSettings::load_flavours(
            &settings.application.configuration_dir,
        );

        let mut default_themes = vec![];
        let mut custom_themes = vec![];

        for flavour in flavours.iter() {
            match flavour {
                Flavour::Light | Flavour::Dark | Flavour::Auto => {
                    default_themes.push(flavour)
                }
                Flavour::Custom(name) => custom_themes.push(name),
            }
        }

        FlavourCurrentCommand::execute(settings.clone(), None);
        println!("");

        println!("Default flavours:");
        for theme in default_themes {
            println!("\t> {}", theme);
        }
        println!("");

        println!("Custom flavours:");
        for theme in custom_themes {
            println!("\t> {}", theme);
        }
    }

    fn help() {
        println!("\t\t--list: lists all flavours");
    }
}
