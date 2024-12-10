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
