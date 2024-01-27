use env_logger::Env;
use std::io::Write;
use std::path::PathBuf;
use smd::commands::{Command, HelpCommand, FlavourCommand, VersionCommand};
use smd::configuration::{Settings, startup};
use smd::utils::invalid_argument_message;


#[tokio::main]
async fn main() {
    let args: Vec<_> = std::env::args().collect();
    env_logger::Builder::from_env(Env::default().default_filter_or("info"))
        .format(|buf, record| writeln!(buf, "{}: {}", record.level(), record.args()))
        .init();

    println!("{:?}", dirs::config_dir().unwrap().join("smd").to_str());
    if args.len() < 2 {
        invalid_argument_message();
    }

    let settings = startup().await;
    match args.len() {
        2 => {
            match args.get(1).unwrap().as_str() {
                "--help" => HelpCommand::execute(settings, None),
                "--version" => VersionCommand::execute(settings, None),
                _ => invalid_argument_message(),
            }
        },
        3 => {
            match args.get(1).unwrap().as_str() {
                "--flavour" => FlavourCommand::execute(settings, Some(args)),
                _ => invalid_argument_message(),
            }
        },
        5 => {
            match args.get(1).unwrap().as_str() {
                "--input" => {
                    if args.get(3).unwrap().as_str() == "--output" {

                    } else {
                        invalid_argument_message();
                    }
                }
                "--output" => {
                    if args.get(3).unwrap().as_str() == "--input" {

                    } else {
                        invalid_argument_message();
                    }
                }
                _ => invalid_argument_message(),
            }
        }
        _ => invalid_argument_message(),
    }
}
