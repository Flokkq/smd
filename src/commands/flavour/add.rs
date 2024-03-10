use crate::commands::Command;
use crate::configuration::Settings;
use crate::markdown::{Flavour, FlavourSettings};
use std::env;
use std::io::Read;
use std::path::PathBuf;
use std::process::exit;

pub struct FlavourAddCommand;

impl Command for FlavourAddCommand {
    fn execute(settings: Settings, _arguments: Option<Vec<String>>) {
        let mut path_str = String::new();
        println!("Please provide the path of the flavour you want to add [*.css]: ");
        std::io::stdin()
            .read_to_string(&mut path_str)
            .unwrap()
            .to_string();

        let mut flavour_str = String::new();
        println!("What do you want the alias of the flavour to be? ");
        std::io::stdin()
            .read_to_string(&mut flavour_str)
            .unwrap()
            .to_string();
        let flavour = Flavour::Custom(flavour_str);

        let path = PathBuf::from(path_str);
        if !path.exists() {
            log::error!("Error retrieving path");
            exit(1);
        }

        if path.is_absolute() {
            FlavourSettings::add_flavour(settings, flavour, path);
        } else {
            let current_dir = match env::current_dir() {
                Ok(current_dir) => current_dir,
                Err(_) => {
                    log::error!("Could not get current path");
                    exit(1);
                }
            };

            FlavourSettings::add_flavour(settings, flavour, current_dir.join(path));
        }
    }

    fn help() {
        println!("\t\t--add: Provide a path to a css file. Note that a copy of the file will be made. Run smd --flavour --update if you made changes to your file");
    }
}

fn send_skeleton_file(name: String) -> Result<(), std::io::Error> {
    Ok(())
}
