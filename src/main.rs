use env_logger::Env;
use smd::commands::{flavour::flavour_base::FlavourCommand, Command, HelpCommand, VersionCommand};
use smd::configuration::startup;
use smd::markdown;
use smd::utils::invalid_argument_message;
use std::io::Write;
use std::path::PathBuf;

#[tokio::main]
async fn main() {
    let args: Vec<_> = std::env::args().collect();
    env_logger::Builder::from_env(Env::default().default_filter_or("info"))
        .format(|buf, record| writeln!(buf, "{}: {}", record.level(), record.args()))
        .init();

    if args.len() < 2 {
        invalid_argument_message();
    }

    let settings = startup().await;
    match args.len() {
        2 => match args.get(1).unwrap().as_str() {
            "--help" => HelpCommand::help(),
            "--version" => VersionCommand::execute(settings, None),
            _ => invalid_argument_message(),
        },
        3 => match args.get(1).unwrap().as_str() {
            "--flavour" => FlavourCommand::execute(settings, Some(args)),
            _ => invalid_argument_message(),
        },
        5 => match args.get(1).unwrap().as_str() {
            "--input" => {
                if args.get(3).unwrap().as_str() == "--output" {
                    markdown::parse(
                        settings,
                        PathBuf::from(args.get(2).unwrap().to_string()),
                        args.get(4).unwrap(),
                        None,
                    )
                    .await;
                } else {
                    invalid_argument_message();
                }
            }
            "--output" => {
                if args.get(3).unwrap().as_str() == "--input" {
                    markdown::parse(
                        settings,
                        PathBuf::from(args.get(4).unwrap().to_string()),
                        args.get(2).unwrap(),
                        None,
                    )
                    .await;
                } else {
                    invalid_argument_message();
                }
            }
            _ => invalid_argument_message(),
        },
        7 => match args.get(1).unwrap().as_str() {
            "--input" => {
                if args.get(3).unwrap().as_str() == "--output"
                    && args.get(5).unwrap().as_str() == "--specific"
                {
                } else {
                    invalid_argument_message();
                }
            }
            "--output" => {
                if args.get(3).unwrap().as_str() == "--input"
                    && args.get(5).unwrap().as_str() == "--specific"
                {
                } else {
                    invalid_argument_message();
                }
            }
            _ => invalid_argument_message(),
        },
        _ => invalid_argument_message(),
    }
}
