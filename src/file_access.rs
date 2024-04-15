use std::io::Write;
use std::path::PathBuf;

use std::fs::{self, OpenOptions};
use std::io::{self};

pub enum WriteOperation {
    Append,
    Write,
}
pub struct FileAccess;

impl FileAccess {
    pub fn read_file(filename: &PathBuf) -> Result<String, ()> {
        return fs::read_to_string(&filename).map_err(|err| {
            log::error!(
                "Could not open file {:?}: {}",
                filename.file_name().unwrap(),
                err,
            )
        });
    }

    pub fn write_file(
        filename: PathBuf,
        content: String,
        operation: WriteOperation,
    ) -> io::Result<()> {
        let file = match operation {
            WriteOperation::Append => {
                OpenOptions::new().append(true).create(true).open(&filename)
            }
            WriteOperation::Write => OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .open(&filename),
        };

        let mut file = match file {
            Ok(file) => file,
            Err(err) => {
                log::error!("Could not open file {:?}: {}", filename, err);
                return Err(err);
            }
        };

        match file.write_all(content.as_bytes()) {
            Ok(_) => Ok(()),
            Err(err) => {
                log::error!(
                    "Could not write to file {:?}: {}",
                    filename.file_name().unwrap_or_default(),
                    err
                );
                Err(err)
            }
        }
    }
}
