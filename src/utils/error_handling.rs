use std::process;

pub fn invalid_argument_message() {
    log::error!("Invalid arguments! Please use smd --help for more information");
    process::exit(1);
}