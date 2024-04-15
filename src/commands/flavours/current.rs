use std::{fs, path::PathBuf};

use crate::{commands::command::Command, file_access::FileAccess};

pub struct FlavourCurrentCommand;

impl Command for FlavourCurrentCommand {
    fn execute(
        settings: crate::configuration::configuration::Settings,
        _arguments: Option<Vec<String>>,
    ) {
        let current_flavour_path = settings
            .application
            .configuration_dir
            .join("current-flavour.css");

        let current_content = match FileAccess::read_file(&current_flavour_path)
        {
            Ok(content) => content,
            Err(_) => {
                println!("Error reading current flavour file.");
                return;
            }
        };

        let flavours_dir =
            settings.application.configuration_dir.join("flavours");

        match determine_current_flavour(&flavours_dir, &current_content) {
            Some(flavour) => println!("Current flavour: {}", flavour),
            None => println!("No matching flavour found."),
        }
    }

    fn help() {
        println!("\t\t--current: Shows the current selected flavour (not implemented)");
    }
}

fn determine_current_flavour(
    flavours_dir: &PathBuf,
    current_content: &str,
) -> Option<String> {
    if let Ok(entries) = fs::read_dir(flavours_dir) {
        for entry in entries.filter_map(|e| e.ok()) {
            let path = entry.path();
            if path.extension().unwrap_or_default() == "css" {
                match FileAccess::read_file(&path) {
                    Ok(content) => {
                        if content == current_content {
                            return path
                                .file_stem()
                                .and_then(|name| name.to_str())
                                .map(String::from);
                        }
                    }
                    Err(_) => {
                        println!("Error reading one of the flavour files.");
                    }
                }
            }
        }
    }
    None
}
