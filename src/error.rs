use std::{fs::remove_dir_all, path::PathBuf, process};

pub fn invalid_argument_message() {
    log::error!(
        "Invalid arguments! Please use smd --help for more information"
    );

    process::exit(1);
}

pub fn initialization_failed(path: &PathBuf) {
    log::error!("Installation failed. Reseting...");
    reset_inizialization(&path, 0);
    process::exit(1);
}

fn reset_inizialization(path: &PathBuf, tries: u16) {
    match remove_dir_all(&path) {
        Ok(_) => {}
        Err(_) => {
            if tries < 10 {
                log::error!("Reseting failed. Retrying {}/10", tries);
                reset_inizialization(&path, tries + 1)
            }

            log::error!(
                "Could not reset. Please manually delete the directory at {}",
                path.to_str().unwrap()
            );
        }
    };
}
