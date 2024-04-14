use log::error;
use rand::Rng;
use std::{
    fs,
    future::Future,
    io::{stdout, Write},
    path::PathBuf,
    process::exit,
    thread,
    time::Duration,
};
use tokio::time::sleep;

use crate::{
    api::{APICommunicator, Template, Theme},
    file_access::{FileAccess, WriteOperation},
};

use super::configuration::APIConfiguration;

pub async fn initialize(
    configuration_dir: &PathBuf,
    api_configuration: &APIConfiguration,
) -> Result<(), anyhow::Error> {
    print!(
        r#"
                               __ 
                              |  \
  _______  ______ ____    ____| $$
 /       \|      \    \  /      $$
|  $$$$$$$| $$$$$$\$$$$\|  $$$$$$$
 \$$    \ | $$ | $$ | $$| $$  | $$
 _\$$$$$$\| $$ | $$ | $$| $$__| $$
|       $$| $$ | $$ | $$ \$$    $$
 \$$$$$$$  \$$  \$$  \$$  \$$$$$$$
                              "#
    );
    print!("\n\n\n");

    Initializer::async_initialization_step(
        "Creating Configuration Directory",
        25,
        || async move {
            fs::create_dir(configuration_dir.clone()).unwrap();
            fs::create_dir(configuration_dir.join("flavours")).unwrap();
            fs::create_dir(configuration_dir.join("extracted")).unwrap();

            let _ = fs::File::create_new(
                configuration_dir.join("current-flavour.css"),
            );

            let _ = std::fs::File::create_new(
                configuration_dir.join("flavours.txt"),
            );
            Ok::<(), anyhow::Error>(())
        },
    )
    .await?;

    Initializer::async_initialization_step(
        "Downloading Standard Themes",
        100,
        || async move {
            let api_communicator =
                APICommunicator::build(api_configuration.clone());
            let themes: Vec<Theme> = api_communicator.get_flavours().await?;

            let mut theme_names: String = String::new();
            for theme in themes {
                let flavour = api_communicator.get_flavour(&theme.name).await?;

                if theme.name != "scaffolding" {
                    theme_names += &format!("{}\n", theme.name.as_str());
                }

                FileAccess::write_file(
                    configuration_dir
                        .join("flavours")
                        .join(format!("{}.css", &theme.name)),
                    flavour,
                    WriteOperation::Write,
                )?;
            }

            FileAccess::write_file(
                configuration_dir.join("flavours.txt"),
                theme_names,
                WriteOperation::Write,
            )?;

            Ok::<(), anyhow::Error>(())
        },
    )
    .await?;

    Initializer::async_initialization_step(
        "Downloading Templates",
        100,
        || async move {
            let api_communicator =
                APICommunicator::build(api_configuration.clone());
            let templates: Vec<Template> =
                api_communicator.get_skeletons().await?;

            for template in templates {
                let skeleton =
                    api_communicator.get_skeleton(&template.name).await?;

                let yaml_string = skeleton.syntax.to_yaml().unwrap();
                FileAccess::write_file(
                    configuration_dir
                        .join("flavours")
                        .join(format!("{}.yaml", &template.name)),
                    yaml_string,
                    WriteOperation::Write,
                )?;
            }

            Ok::<(), anyhow::Error>(())
        },
    )
    .await?;

    Initializer::initialization_step("Applying Default Settings", 100, || {
        let standard_flavour = FileAccess::read_file(
            configuration_dir.join("flavours").join("dark.css"),
        )
        .unwrap();

        FileAccess::write_file(
            configuration_dir.join("current-flavour.css"),
            standard_flavour,
            WriteOperation::Write,
        )?;

        Ok::<(), anyhow::Error>(())
    })?;

    Initializer::async_initialization_step(
        "Verifying Setup Correctness",
        50,
        || async move {
            let api_communicator =
                APICommunicator::build(api_configuration.clone());
            let themes: Vec<Theme> =
                api_communicator.get_flavours().await.unwrap();
            let templates: Vec<Template> =
                api_communicator.get_skeletons().await.unwrap();

            let mut expected_files = vec!["current-flavour.css".to_string()];

            for theme in themes {
                expected_files.push(format!("flavours/{}.css", theme.name));
            }

            for template in templates {
                expected_files.push(format!("flavours/{}.css", template.name));
            }

            for file in expected_files {
                let file_path = configuration_dir.join(file);

                if !file_path.exists() {
                    error!("Initialization failed");
                    exit(1);
                }

                let metadata = fs::metadata(&file_path)
                    .map_err(|_| {
                        error!("Initialization failed");
                        exit(1);
                    })
                    .unwrap();

                if metadata.len() == 0 {
                    error!("Initialization failed");
                    exit(1);
                }
            }
            Ok::<(), anyhow::Error>(())
        },
    )
    .await?;

    Ok(())
}

struct Initializer;

impl Initializer {
    fn print_progress_bar(description: &str, duration: u64) {
        let progress_bar_len = 32;
        let mut rng = rand::thread_rng();

        println!("{}", description);

        print!("|");
        for _ in 0..progress_bar_len {
            print!("-");
        }
        println!("|");

        print!("|");
        for _ in 0..progress_bar_len {
            print!("=");
            stdout().flush().unwrap();
            thread::sleep(Duration::from_millis(
                duration + rng.gen_range(0..=duration),
            ));
        }
        println!("| Done\n");
    }

    async fn print_progress_bar_async(description: &str, duration: u64) {
        let progress_bar_len = 32;
        let mut rng = rand::thread_rng();

        println!("{}", description);

        print!("|");
        for _ in 0..progress_bar_len {
            print!("-");
        }
        println!("|");

        print!("|");
        for _ in 0..progress_bar_len {
            print!("=");
            stdout().flush().unwrap();
            sleep(Duration::from_millis(
                duration + rng.gen_range(0..=duration),
            ))
            .await;
        }
        println!("| Done\n");
    }

    pub fn initialization_step<F, E>(
        description: &str,
        duration: u64,
        callback: F,
    ) -> Result<(), E>
    where
        F: Fn() -> Result<(), E>,
    {
        callback()?;
        Self::print_progress_bar(description, duration);
        Ok(())
    }

    pub async fn async_initialization_step<F, Fut, E>(
        description: &str,
        duration: u64,
        callback: F,
    ) -> Result<(), E>
    where
        F: Fn() -> Fut,
        Fut: Future<Output = Result<(), E>>,
    {
        callback().await?;
        Self::print_progress_bar_async(description, duration).await;
        Ok(())
    }
}
