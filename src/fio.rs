use std::fs;
use std::fs::{File};
use std::io::Write;
use std::path::PathBuf;
use std::result::Result;
use std::process::exit;

pub fn read_file(filename: PathBuf) -> Result<String, ()> {
    println!("INFO Reading file: {:?}", filename.file_name().unwrap());
    return fs::read_to_string(filename.file_name().unwrap()).map_err(|err| {
        eprintln!("ERROR: could not open file {:?}: {err}", filename.file_name().unwrap());
    })
}

pub fn write_file(filename: PathBuf, content: &str) {
    println!("INFO Writing file: {:?}", filename.file_name().unwrap());
    let mut file = match File::create(&filename) {
        Ok(file) => file,
        Err(err) => {
            eprintln!("ERROR: could not create file {:?}: {}", filename.file_name().unwrap(), err);
            exit(1);
        }
    };

    file.write_all(content.as_bytes()).map_err(|err| {
        eprintln!("ERROR: could not write to file {:?}: {}", filename.file_name().unwrap(), err)
    }).expect("TODO: panic message");
}
