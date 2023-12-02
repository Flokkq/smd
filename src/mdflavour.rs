use crate::{fio, config};

pub enum MdFlavour {
    Light,
    Dark,
    Auto
}

impl MdFlavour {
    pub fn to_string(&self) -> String {
        match self {
            MdFlavour::Light => "light".to_string(),
            MdFlavour::Dark => "dark".to_string(),
            MdFlavour::Auto => "auto".to_string()
        }
    }
}
pub fn set_md_flavour(flavour: MdFlavour) {
    let filepath = config::get_path_to_config_file();

    fio::write_file(filepath, &*flavour.to_string())
}

