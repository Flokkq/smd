use std::env;
use std::process::{Command, exit};
use std::result::Result;
use std::path::PathBuf;
use std::error::Error;
use std::fs::File;
use std::fs;

pub fn check_requirements() {
    // println!("INFO Checking requirements...");

    let npm_check = Command::new("npm")
        .arg("ls")
        .arg("-g")
        .output();

    match npm_check {
        Ok(output) => {
            let response = String::from_utf8_lossy(&output.stdout);
            if !response.contains(" github-markdown-css") {
                // println!("INFO Installing github-markdown-css...");
                match Command::new("npm")
                    .arg("install")
                    .arg("-g")
                    .arg("github-markdown-css")
                    .output() {
                    Ok(_) => /*println!("INFO All requirements are fulfilled! Continuing...")*/ (),
                    Err(err) => {
                        eprintln!("ERROR installing github-markdown-css: {}", err);
                        exit(1);
                    }
                }
            }
            //println!("INFO All requirements are fulfilled! Continuing...");
        }
        Err(err) => {
            eprintln!("ERROR checking requirements: {}", err)
        }
    }
}

pub fn validate_inputs(filename: &str, output_type: &str, specific_type: Option<&str>) -> Result<(), Box<dyn Error>> {
    if filename.is_empty() {
        return Err("ERROR: no input file specified".into());
    }

    let current_dir = env::current_dir().expect("ERROR resolving current directory");
    if !current_dir.join(filename).exists() {
        let error_msg = format!("ERROR: file {} does not exist", filename);
        return Err(error_msg.into())
    }

    match output_type {
        "html" | "pdf" | "img" => (),
        _ => return Err("ERROR: invalid output type".into())
    }

    match specific_type {
        Some(str) => {
            match str {
                "jpeg" | "png" | "webp" => (),
                &_ => return Err("ERROR: invalid specific type".into())
            }
        },
        None => ()
    }

    return Ok(());
}

pub fn get_path_to_config_file() -> PathBuf {
    let home_dir = dirs::home_dir().expect("ERROR getting home directory");
    #[cfg(unix)]
        let dir_path = home_dir.join("Library/Smd");
    #[cfg(windows)]
        let dir_path = home_dir.join("AppData/Roaming/Smd");
     return dir_path.join("md_flavour.txt");
}

pub fn create_config_file() {
    let file_path = get_path_to_config_file();
    fs::create_dir(&file_path.parent().unwrap()).expect("ERROR creating directory");
    File::create(file_path).expect("ERROR creating file");
}
