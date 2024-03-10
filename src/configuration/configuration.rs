use crate::configuration::{initialize::initialize, verify_initial_setup};
use crate::markdown::{Flavour, FlavourSettings};
use dotenv::dotenv;
use std::path::PathBuf;
use walkdir::WalkDir;

pub struct Settings {
    pub appearance: Appearance,
    pub config_folder: PathBuf,
}

pub struct Appearance {
    pub flavour_settings: FlavourSettings,
    pub path: PathBuf,
}

impl Settings {
    pub fn new(appearance: Appearance, config_folder: PathBuf) -> Settings {
        Settings {
            appearance,
            config_folder,
        }
    }
}

pub async fn startup() -> Settings {
    match verify_initial_setup() {
        Ok(_) => {
            log::info!("Initial setup verified");
            dotenv().ok();
        }
        Err(err) => {
            log::error!("Could not verify initial setup: {:?}", err);
            println!("Do you want to (re)initialize smd[y/n]? ");

            let mut input = String::new();
            if std::io::stdin()
                .read_line(&mut input)
                .unwrap()
                .to_string()
                .trim()
                != "y"
            {
                todo!("fix false if statement!");
                log::info!("(re)initializing smd...");
                initialize().await;
            }
        }
    }

    let config_folder = get_config_folder();
    let flavour_settings = FlavourSettings::new(Flavour::Auto, &config_folder);
    let appearance = Appearance::new(flavour_settings, config_folder.clone());
    let mut settings = Settings::new(appearance, config_folder);
    let flavour = FlavourSettings::get_current_flavour(&settings);

    settings.appearance.flavour_settings.flavour = flavour;
    settings
}

impl Appearance {
    pub fn new(flavour_settings: FlavourSettings, path: PathBuf) -> Appearance {
        Appearance {
            flavour_settings,
            path,
        }
    }
}

fn get_config_folder() -> PathBuf {
    dirs::config_dir().unwrap().join("smd")
}

pub fn get_flavours_from_config_folder(config_folder: PathBuf) -> Result<Vec<String>, ()> {
    let mut flavours: Vec<_> = vec![];

    for entry in WalkDir::new(config_folder) {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.is_file() {
            if let Some(file_name) = path.file_stem() {
                if let Some(file_name_str) = file_name.to_str() {
                    flavours.push(file_name_str.to_lowercase());
                }
            }
        }
    }

    Ok(flavours)
}
