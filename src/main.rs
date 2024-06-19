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
            // TODO Add short version of commands
            "-h" => HelpCommand::execute(settings, None),
            "--help" => HelpCommand::execute(settings, None),
            "--version" => VersionCommand::execute(settings, None),
            _ => HelpCommand::execute(settings, None),
        },
        3 | 4 => match args.get(1).unwrap().as_str() {
            "--flavour" => FlavourCommand::execute(settings, Some(args)),
            _ => HelpCommand::execute(settings, None),
        },
        5 => {
            if let Some(parsed) = parse_arguments(&args, 5) {
                parse(
                    settings,
                    PathBuf::from(parsed.input),
                    &parsed.output,
                    parsed.specific.as_deref(),
                    parsed.flavour.as_deref(),
                )
                .await;
            } else {
                HelpCommand::execute(settings, None);
            }
        }
        7 => {
            if let Some(parsed) = parse_arguments(&args, 7) {
                parse(
                    settings,
                    PathBuf::from(parsed.input),
                    &parsed.output,
                    parsed.specific.as_deref(),
                    parsed.flavour.as_deref(),
                )
                .await;
            } else {
                HelpCommand::execute(settings, None);
            }
        }
        9 => {
            if let Some(parsed) = parse_arguments(&args, 9) {
                parse(
                    settings,
                    PathBuf::from(parsed.input),
                    &parsed.output,
                    parsed.specific.as_deref(),
                    parsed.flavour.as_deref(),
                )
                .await;
            } else {
                HelpCommand::execute(settings, None);
            }
        }
        _ => HelpCommand::execute(settings, None),
    }
}

fn parse_arguments(
    args: &[String],
    arg_count: usize,
) -> Option<ArgParseResult> {
    let mut input = String::new();
    let mut output = String::new();
    let mut specific = None;
    let mut flavour = None;

    let mut i = 1;
    while i < arg_count {
        match args[i].as_str() {
            "--input" if i + 1 < arg_count => input = args[i + 1].clone(),
            "--output" if i + 1 < arg_count => output = args[i + 1].clone(),
            "--specific" if i + 1 < arg_count => {
                specific = Some(args[i + 1].clone())
            }
            "--apply-flavour" if i + 1 < arg_count => {
                flavour = Some(args[i + 1].clone())
            }
            _ => return None,
        }
        i += 2;
    }

    Some(ArgParseResult {
        input,
        output,
        specific,
        flavour,
    })
}

struct ArgParseResult {
    input: String,
    output: String,
    specific: Option<String>,
    flavour: Option<String>,
}
