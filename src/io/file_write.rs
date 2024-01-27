use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::process::exit;

pub fn write_file(filename: PathBuf, content: String) {
    let mut file = match File::create(&filename) {
        Ok(file) => file,
        Err(err) => {
            log::error!(
                "Could not create file {:?}: {}",
                filename,
                err,
            );
            exit(1);
        }
    };

    file.write_all(content.as_bytes()).map_err(|err| {
        log::error!(
            "Could not write file {:?}: {}",
            filename.file_name().unwrap(),
            err
        )
    }).expect("");
}