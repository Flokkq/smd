use std::fs;
use std::path::PathBuf;

pub fn read_file(filename: PathBuf) -> Result<String, ()> {
    return fs::read_to_string(&filename).map_err(|err| {
        log::error!(
            "Could not open file {:?}: {}",
            filename.file_name().unwrap(),
            err,
        )
    });
}
