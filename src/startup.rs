use std::{
    io::{stdin, stdout, Write},
    process::exit,
};

use crate::{
    commands::{command::Command, help::HelpCommand},
    configuration::{
        configuration::{get_configuration, Settings},
        env_check::{is_update_available, verify_initial_setup},
        initialize::initialize,
        updater::update,
    },
    error::initialization_failed,
};

pub async fn startup() -> Result<Settings, anyhow::Error> {
    let settings = get_configuration().unwrap();

    if let Err(_) =
        verify_initial_setup(&settings.application.configuration_dir)
    {
        if initialize(&settings.application.configuration_dir, &settings.api)
            .await
            .is_err()
        {
            initialization_failed(&settings.application.configuration_dir)
        };

        println!("\nWelcome to Sweet Markdown!");
        HelpCommand::execute(settings.clone(), None);
        exit(1);
    } else if let Some(update_infos) =
        is_update_available(&settings).await.ok().flatten()
    {
        print!("An update is available! Do you want to update? [y/n]: ");
        stdout().flush().unwrap();

        let mut choice = String::new();
        stdin()
            .read_line(&mut choice)
            .expect("Failed reading input");

        if choice.trim().to_lowercase() == "y" {
            match update(&settings.application.configuration_dir, update_infos)
                .await
            {
                Ok(_) => {
                    println!("\nSweet Markdown updated successfully!");
                    HelpCommand::execute(settings.clone(), None);
                    exit(0);
                }
                Err(e) => println!("Update failed"),
            }
        }
    }

    Ok(settings)
}

