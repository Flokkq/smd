use std::io::Read;
use std::path::PathBuf;
use crate::commands::Command;
use crate::configuration::Settings;
use crate::markdown::Flavour;

pub struct FlavourAddCommand;

impl Command for FlavourAddCommand {
    fn execute(settings: Settings, _arguments: Option<Vec<String>>) {
        let mut path_str = String::new();
        println!("Please provide the path of the flavour you want to add [*.css]: ");
        std::io::stdin().read_to_string(&mut path_str).unwrap().to_string();

        let mut flavour_str = String::new();
        println!("What do you want the alias fo the flavour to be? ");
        std::io::stdin().read_to_string(& mut flavour_str).unwrap().to_string();

        let flavour = Flavour::Custom(flavour_str);

        let path = PathBuf::from(path_str);
        if path.is_absolute() {
            Flavour::add_flavour(settings, flavour, path);
        } else {

        }
    }
}