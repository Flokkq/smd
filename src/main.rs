use env_logger::Env;
use smd::{
    commands::{
        command::Command, flavours::flavour::FlavourCommand, help::HelpCommand,
        version::VersionCommand,
    },
    error::invalid_argument_message,
    markdown::parser::parse,
    startup::startup,
};
use std::{io::Write, path::PathBuf};

#[tokio::main]
async fn main() {
    let args: Vec<_> = std::env::args().collect();
    env_logger::Builder::from_env(Env::default().default_filter_or("smd=info"))
        .format(|buf, record| {
            writeln!(buf, "{}: {}", record.level(), record.args())
        })
        .init();

    if args.len() < 1 || args.len() > 7 {
        invalid_argument_message();
    }

    let settings = startup().await.unwrap();
    match args.len() {
        1 => {
            HelpCommand::execute(settings, None);
        }
        2 => match args.get(1).unwrap().as_str() {
            "--help" => HelpCommand::execute(settings, None),
            "--version" => VersionCommand::execute(settings, None),
            _ => HelpCommand::execute(settings, None),
        },
        3 | 4 => match args.get(1).unwrap().as_str() {
            "--flavour" => FlavourCommand::execute(settings, Some(args)),
            _ => HelpCommand::execute(settings, None),
        },
        5 => match args.get(1).unwrap().as_str() {
            "--input" => {
                if args.get(3).unwrap().as_str() == "--output" {
                    parse(
                        settings,
                        PathBuf::from(args.get(2).unwrap().to_string()),
                        args.get(4).unwrap(),
                        None,
                    )
                    .await;
                } else {
                    HelpCommand::execute(settings, None);
                }
            }
            "--output" => {
                if args.get(3).unwrap().as_str() == "--input" {
                    parse(
                        settings,
                        PathBuf::from(args.get(4).unwrap().to_string()),
                        args.get(2).unwrap(),
                        None,
                    )
                    .await;
                } else {
                    HelpCommand::execute(settings, None);
                }
            }
            _ => HelpCommand::execute(settings, None),
        },
        7 => match args.get(1).unwrap().as_str() {
            "--input" => {
                if args.get(3).unwrap().as_str() == "--output"
                    && args.get(5).unwrap().as_str() == "--specific"
                {
                    parse(
                        settings,
                        PathBuf::from(args.get(2).unwrap().to_string()),
                        args.get(4).unwrap(),
                        Some(args.get(6).unwrap()),
                    )
                    .await;
                } else {
                    HelpCommand::execute(settings, None);
                }
            }
            "--output" => {
                if args.get(3).unwrap().as_str() == "--input"
                    && args.get(5).unwrap().as_str() == "--specific"
                {
                    parse(
                        settings,
                        PathBuf::from(args.get(4).unwrap().to_string()),
                        args.get(2).unwrap(),
                        Some(args.get(6).unwrap()),
                    )
                    .await;
                } else {
                    HelpCommand::execute(settings, None);
                }
            }
            _ => HelpCommand::execute(settings, None),
        },
        _ => HelpCommand::execute(settings, None),
    }
}
