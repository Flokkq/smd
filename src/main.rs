mod smd;

use std::env;
use std::process;
use crate::smd::MdFlavour;

const VERSION: &str = "0.0.1";
const RED_CODE: &str = "\x1b[31m";
const BLUE_CODE: &str = "\x1b[34m";
const MAGENTA_CODE: &str = "\x1b[35m";
const NORMAL_CODE: &str = "\x1b[0m";

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        invalid_argument_message();
    }

    match args.len() {

        2 => {
            match args.get(1).unwrap().as_str() {
                "--help" => {
                    println!("{}Usage:{} {}smd{} {}[input_file] [output_type] [*specific_type]{}", RED_CODE, NORMAL_CODE,
                           BLUE_CODE, NORMAL_CODE, MAGENTA_CODE, NORMAL_CODE);
                    println!("{}Options:{}", RED_CODE, NORMAL_CODE);
                    println!("  {}--help{}\t\t\tDisplay this information", MAGENTA_CODE, NORMAL_CODE);
                    println!("  {}--init{}\t\t\tInitialize smd. Needed for the first use.", MAGENTA_CODE, NORMAL_CODE);
                    println!("  {}--flavour{}\t\t\tSet flavour for the md files.", MAGENTA_CODE, NORMAL_CODE);
                    println!("  {}--version{}\t\t\tDisplay version information", MAGENTA_CODE, NORMAL_CODE);
                    println!("  {}--output [file]{}\t\tSpecify output type", MAGENTA_CODE, NORMAL_CODE);
                    println!("  {}--input [file]{}\t\tSpecify input file", MAGENTA_CODE, NORMAL_CODE);
                    println!("  {}--specific [type]{}\t\tSpecify specific output type", MAGENTA_CODE, NORMAL_CODE);
                }
                "--version" => {
                    println!("{}SMD{} version {}\n", BLUE_CODE, NORMAL_CODE, VERSION);
                }
                "--init" => {
                    println!("INFO Initializing {}smd{}...", BLUE_CODE, NORMAL_CODE);
                    smd::check_requirements();
                    smd::set_md_flavour(MdFlavour::Auto);
                    println!("{}smd{} initialized! Use {}smd{} --help ", BLUE_CODE, NORMAL_CODE, BLUE_CODE, NORMAL_CODE);
                }
                "--flavour" => {
                    // TODO: fix problem with stdin reading before last print
                    println!("What flavour do you want to use?");
                    println!("  {}1{} > dark", MAGENTA_CODE, NORMAL_CODE);
                    println!("  {}2{} > light", MAGENTA_CODE, NORMAL_CODE);
                    println!("  {}3{} > auto", MAGENTA_CODE, NORMAL_CODE);
                    println!("flavour: ");

                    let mut flavour = String::new();
                    std::io::stdin().read_line(&mut flavour).expect("ERROR Failed reading input");

                    match flavour.trim().as_str() {
                        "1" | "dark" => smd::set_md_flavour(MdFlavour::Dark),
                        "2" | "light" => smd::set_md_flavour(MdFlavour::Light),
                        "3" | "auto" => smd::set_md_flavour(MdFlavour::Auto),
                        _ => {
                            println!("{}Invalid flavour!{}", RED_CODE, NORMAL_CODE);
                            process::exit(1);
                        }
                    }

                }
                _ => invalid_argument_message()
            }
        }
        5 => {
            match args.get(1).unwrap().as_str() {
                "--input" => {
                    if args.get(3).unwrap().as_str() == "--output" {
                        smd::parse_md(args.get(2).unwrap().as_str(), args.get(4).unwrap().as_str(), None);
                    } else {
                        invalid_argument_message();
                    }
                }
                "--output" => {
                    if args.get(3).unwrap().as_str() == "--input" {
                        smd::parse_md(args.get(4).unwrap().as_str(), args.get(2).unwrap().as_str(), None);
                    } else {
                        invalid_argument_message();
                    }
                }
                _ => invalid_argument_message()
            }

        }
        7 => {
            match args.get(1).unwrap().as_str() {
                "--input" => {
                    if args.get(3).unwrap().as_str() == "--output" && args.get(5).unwrap().as_str() == "--specific" {
                        smd::parse_md(args.get(2).unwrap().as_str(), args.get(4).unwrap().as_str(), Some(args.get(6).unwrap().as_str()));
                    } else {
                        invalid_argument_message();
                    }
                }
                "--output" => {
                    if args.get(3).unwrap().as_str() == "--input" && args.get(5).unwrap().as_str() == "--specific" {
                        smd::parse_md(args.get(4).unwrap().as_str(), args.get(2).unwrap().as_str(), Some(args.get(6).unwrap().as_str()));
                    } else {
                        invalid_argument_message();
                    }
                }
                _ => invalid_argument_message()
            }
        }
        _ => invalid_argument_message()
    }
}

fn invalid_argument_message() {
    println!("Invalid arguments! Please use smd --help for more information");
    process::exit(1);
}
