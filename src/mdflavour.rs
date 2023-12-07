use crate::{fio, config};
use std::process::exit;

pub enum MdFlavour {
    Light,
    Dark,
    Auto
}

impl MdFlavour {
    pub fn to_string(&self) -> String {
        match self {
            MdFlavour::Light => "-light".to_string(),
            MdFlavour::Dark => "-dark".to_string(),
            MdFlavour::Auto => "".to_string()
        }
    }
}

pub fn set_md_flavour(flavour: MdFlavour) {
    let filepath = config::get_path_to_config_file();
    fio::write_file(filepath, &flavour.to_string())
}
pub fn get_md_flavour() -> String {
    let filepath = config::get_path_to_config_file();
    let content = match fio::read_file(filepath) {
        Ok(content) => content,
        Err(_) => {
            eprintln!("ERROR retrieving markdown flavour");
            exit(1);
        }
    };
    content
}