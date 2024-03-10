use crate::commands::Command;
use crate::configuration::Settings;
use crate::markdown::FlavourSettings;

pub struct FlavourSetCommand;

impl Command for FlavourSetCommand {
    fn execute(mut settings: Settings, arguments: Option<Vec<String>>) {
        let flavours = FlavourSettings::load_flavours(&settings);

        println!("Which flavour would you like to use?");
        for (i, flavour) in flavours.iter().enumerate() {
            println!("\t{} > {}", i + 1, flavour.to_string().to_lowercase());
        }

        loop {
            println!("flavour: ");

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
                match flavours.get(selected_flavour - 1) {
                    Some(flavour) => {
                        settings.appearance.flavour_settings.flavour = flavour.clone();
                        FlavourSettings::set_flavour(settings);
                    }
                    None => {
                        log::error!("Failed setting flavour!");
                        std::process::exit(1);
                    }
                };
                break;
            }
            log::warn!("Invalid input! Please provide a valid number within the range.");
        }
    }

    fn help() {
        println!("\t\t--set: Set a flavour for the markdown file");
    }
}
