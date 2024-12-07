use thiserror::Error as ThisError;

/// Library related errors that are exposing to the rest of the workspaces.
#[derive(Debug, ThisError)]
pub enum Error {}

/// Result type of the core library.
pub type Result<T> = core::result::Result<T, Error>;
