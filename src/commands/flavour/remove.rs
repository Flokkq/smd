use crate::commands::Command;
use crate::configuration::Settings;
use crate::markdown::Flavour;

pub struct FlavourRemoveCommand;

impl Command for FlavourRemoveCommand {
    fn execute(mut settings: Settings, _arguments: Option<Vec<String>>) {
        let flavours = Flavour::load_flavours(&settings);

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
                        if settings.flavour == *flavour {
                            settings.flavour = Flavour::Auto;
                        }
                        Flavour::remove_flavour(settings, flavour.clone());
                    },
                    None => {
                        log::error!("Failed removing flavour!");
                        std::process::exit(1);
                    }
                };
                break;
            }
            log::warn!("Invalid input! Please provide a valid number within the range.");
        }


        let flavour = Flavour::Auto;

    }
}