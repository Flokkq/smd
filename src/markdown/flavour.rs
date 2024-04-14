use std::{
    convert::Infallible,
    fmt::Formatter,
    fs::{self, remove_file, OpenOptions},
    io::{BufRead, BufReader, Write},
    path::PathBuf,
    process::exit,
    str::FromStr,
};

use crate::{
    file_access::{FileAccess, WriteOperation},
    skeleton::{Skeleton, SkeletonSyntax},
};

pub enum Flavour {
    Light,
    Dark,
    Auto,
    Custom(String),
}

pub struct FlavourSettings {
    pub flavour: Flavour,
    pub path_to_yaml_config: PathBuf,
    pub path_to_css_config: PathBuf,
}

impl FlavourSettings {
    pub fn new(
        flavour: Flavour,
        configuration_dir: &PathBuf,
    ) -> FlavourSettings {
        let path_to_css_config =
            configuration_dir.join(flavour.to_css_format());
        let path_to_yaml_config =
            configuration_dir.join(flavour.to_yaml_format());

        FlavourSettings {
            flavour,
            path_to_yaml_config,
            path_to_css_config,
        }
    }

    pub fn load_flavours(configuration_dir: &PathBuf) -> Vec<Flavour> {
        let mut flavours = vec![Flavour::Light, Flavour::Dark, Flavour::Auto];
        let content =
            FileAccess::read_file(configuration_dir.join("flavours.txt"));

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

    pub fn set_flavour(configuration_dir: &PathBuf, flavour: Flavour) {
        let content = FileAccess::read_file(
            configuration_dir.join(flavour.to_css_format()),
        )
        .unwrap();

        FileAccess::write_file(
            configuration_dir.join("current-flavour.css"),
            content,
            WriteOperation::Write,
        )
        .unwrap();
    }

    pub fn new_flavour(
        configuration_dir: &PathBuf,
        flavour: Flavour,
        template_flavour: Flavour,
    ) {
        let skeleton_file_path = if template_flavour == Flavour::Auto {
            "flavours/light.yaml".to_string()
        } else {
            template_flavour.to_yaml_format()
        };

        let skeleton =
            FileAccess::read_file(configuration_dir.join(skeleton_file_path))
                .unwrap();

        let current_dir = std::env::current_dir().unwrap();
        FileAccess::write_file(
            current_dir.join(&format!("{}.yaml", flavour.to_string())),
            skeleton,
            WriteOperation::Write,
        )
        .unwrap()
    }

    pub fn add_flavour(
        configuration_dir: &PathBuf,
        flavour_settings: FlavourSettings,
        path_to_flavour: PathBuf,
    ) {
        Self::apply_flavour_changes(
            configuration_dir,
            &flavour_settings,
            path_to_flavour,
        );

        let flavour_str = format!("{}\n", flavour_settings.flavour.to_string());
        FileAccess::write_file(
            configuration_dir.join("flavours.txt"),
            flavour_str,
            WriteOperation::Append,
        )
        .unwrap()
    }

    pub fn remove_flavour(configuration_dir: &PathBuf, flavour: Flavour) {
        let flavours_file_path = configuration_dir.join("flavours.txt");

        match flavour {
            Flavour::Dark | Flavour::Light | Flavour::Auto => {
                log::warn!("Cannot remove standard theme");
                exit(1);
            }
            _ => {}
        }

        if let Ok(file) = fs::File::open(configuration_dir.join("flavour.txt"))
        {
            let lines: Vec<String> = BufReader::new(file)
                .lines()
                .filter_map(|line| line.ok())
                .collect();

            if let Ok(mut file) = OpenOptions::new()
                .write(true)
                .truncate(true)
                .open(&flavours_file_path)
            {
                for line in lines.iter() {
                    if line.to_string() != flavour.to_string() {
                        writeln!(file, "{}", line)
                            .expect("Failed to write flavour to file");
                    }
                }
            }
        }

        remove_file(flavour.to_css_format()).expect("Failed removing files");
        remove_file(flavour.to_yaml_format()).expect("Failed removing files");
    }

    pub fn update_flavour(
        configuration_dir: &PathBuf,
        flavour_settings: FlavourSettings,
        path_to_flavour: PathBuf,
        old_flavour: Option<Flavour>,
    ) {
        match old_flavour.clone() {
            Some(flavour) => match flavour {
                Flavour::Dark | Flavour::Light | Flavour::Auto => {
                    log::warn!("Cannot rename standard theme");
                    exit(1);
                }
                _ => {}
            },
            None => {}
        }

        Self::apply_flavour_changes(
            configuration_dir,
            &flavour_settings,
            path_to_flavour,
        );

        if let Some(flavour) = old_flavour {
            let mut flavours = Self::load_flavours(configuration_dir);

            flavours.retain(|f| !f.eq(&flavour));
            flavours.push(flavour_settings.flavour);

            let mut flavour_names = String::new();
            for flavour in flavours.iter() {
                flavour_names += &format!("{}\n", flavour.to_string());
            }

            FileAccess::write_file(
                configuration_dir.join("flavours.txt"),
                flavour_names,
                WriteOperation::Write,
            )
            .unwrap()
        }
    }

    pub fn edit_flavour(configuration_dir: &PathBuf, flavour: Flavour) {
        let flavour_file_path =
            configuration_dir.join(flavour.to_yaml_format());

        let target_path = std::env::current_dir()
            .unwrap()
            .join(format!("{}.yaml", flavour.to_string()));

        fs::copy(&flavour_file_path, &target_path).unwrap();
    }

    fn apply_flavour_changes(
        configuration_dir: &PathBuf,
        flavour_settings: &FlavourSettings,
        path_to_flavour: PathBuf,
    ) {
        let file_path = if path_to_flavour.is_absolute() {
            path_to_flavour
        } else {
            std::env::current_dir().unwrap().join(path_to_flavour)
        };

        let content = FileAccess::read_file(file_path).unwrap();
        let skeleton =
            Skeleton::new(SkeletonSyntax::from_yaml(&content).unwrap());

        FileAccess::write_file(
            flavour_settings.path_to_css_config.clone(),
            skeleton.to_css(configuration_dir).unwrap(),
            WriteOperation::Write,
        )
        .unwrap();

        FileAccess::write_file(
            flavour_settings.path_to_yaml_config.clone(),
            content,
            WriteOperation::Write,
        )
        .unwrap();
    }
}

impl Flavour {
    pub fn to_css_format(&self) -> String {
        format!("flavours/{}.css", self)
    }

    pub fn to_yaml_format(&self) -> String {
        format!("flavours/{}.yaml", self)
    }
}

impl PartialEq for Flavour {
    fn eq(&self, other: &Self) -> bool {
        return self.to_string() == other.to_string();
    }
}

impl FromStr for Flavour {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "light" => Ok(Flavour::Light),
            "dark" => Ok(Flavour::Dark),
            "auto" => Ok(Flavour::Auto),
            _ => Ok(Flavour::Custom(s.to_string())),
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
