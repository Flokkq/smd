use crate::configuration::Settings;
use crate::io::{read_file, write_file};
use std::fmt::{Error, Formatter};
use std::path::PathBuf;

#[derive(PartialEq)]
pub enum Flavour {
    Light,
    Dark,
    Auto,
    Custom(String),
}

pub struct FlavourSettings {
    pub flavour: Flavour,
    pub path: PathBuf,
}

impl FlavourSettings {
    pub fn new(flavour: Flavour, path: &PathBuf) -> FlavourSettings {
        let flavour_string = flavour.to_string();
        let path_to_flavour = path.join(format!("{}.css", flavour_string));

        FlavourSettings {
            flavour,
            path: path_to_flavour,
        }
    }

    pub fn load_flavours(settings: &Settings) -> Vec<Flavour> {
        let mut flavours = vec![Flavour::Light, Flavour::Dark, Flavour::Auto];
        let content = read_file(settings.config_folder.join("flavours.txt"));

        match content {
            Ok(content) => {
                for line in content.lines().skip(3) {
                    flavours.push(Flavour::Custom(line.to_string()));
                }
            }
            Err(_) => {}
        }

        flavours
    }

    pub fn set_flavour(settings: Settings) {
        write_file(
            settings.config_folder.join("current-flavour.css"),
            settings.appearance.flavour_settings.flavour.to_string(),
        );
    }

    pub fn get_current_flavour(settings: &Settings) -> Flavour {
        let mut flavour = settings.appearance.flavour_settings.flavour.clone();
        let content = read_file(settings.config_folder.join("current-flavour.css"));

        match content {
            Ok(content) => {
                flavour = Flavour::from_str(content.trim());
            }
            Err(err) => log::error!("Error reading flavour file: {:?}", err),
        }
        flavour
    }

    pub fn add_flavour(settings: Settings, flavour: Flavour, path: PathBuf) {
        let mut flavours = Self::load_flavours(&settings);
        flavours.push(flavour);

        let mut content = String::new();
        for flavour in flavours {
            content.push_str(&format!("{}\n", flavour));
        }

        write_file(settings.config_folder.join("flavours.txt"), content);
    }

    pub fn remove_flavour(mut settings: Settings, flavour: Flavour) {
        todo!()
    }

    pub fn update_flavour(_settings: Settings) {
        todo!()
    }
}

impl Flavour {
    pub fn from_str(name: &str) -> Flavour {
        match name {
            "Light" => Flavour::Light,
            "Dark" => Flavour::Dark,
            "Auto" => Flavour::Auto,
            _ => Flavour::Custom(name.to_string()),
        }
    }
}

impl std::fmt::Display for Flavour {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Flavour::Light => write!(f, "light"),
            Flavour::Dark => write!(f, "dark"),
            Flavour::Auto => write!(f, "auto"),
            Flavour::Custom(name) => write!(f, "{}", name),
        }
    }
}

impl Clone for Flavour {
    fn clone(&self) -> Self {
        match self {
            Flavour::Light => Flavour::Light,
            Flavour::Dark => Flavour::Dark,
            Flavour::Auto => Flavour::Auto,
            Flavour::Custom(name) => Flavour::Custom(name.clone()),
        }
    }
}
