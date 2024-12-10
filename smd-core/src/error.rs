use thiserror::Error as ThisError;

/// Library related errors that are exposing to the rest of the workspaces.
#[derive(Debug, ThisError)]
pub enum Error {
	#[error("IO error: `{0}`")]
	IoError(#[from] std::io::Error),

	#[error("Serialization error: `{0}`")]
	SerializeError(String),

	#[error("Deserialization error: `{0}`")]
	DeserializeError(String),
}

/// Result type of the core library.
pub type Result<T> = core::result::Result<T, Error>;

#[cfg(test)]
mod test {
	use std::{
		fs,
		io::ErrorKind,
		path::PathBuf,
	};

	use super::*;

	fn mock_function() -> super::Result<String> {
		let path = PathBuf::from("path_that_does_not_exist");
		Ok(fs::read_to_string(path)?)
	}

	#[test]
	fn throw_parse_error() {
		let actual_error = mock_function().expect_err("Expected error");

		match actual_error {
			Error::IoError(e) => {
				assert_eq!(e.kind(), ErrorKind::NotFound);
			}
			_ => {
				panic!("Unexpected error type");
			}
		}
	}
}
