/// Library related error management
pub mod error;

/// IO with for custom structs
pub mod fs;

/// Config file parser.
pub mod config;

/// Convert html to other file formats.
pub mod convert;

/// Wrapper for the headless-chrome crate
pub mod browser;

/// Wrapper to interact with markdown-stores
pub mod vendor;

/// Default configuration file.
pub const DEFAULT_CONFIG: &str = "smd/config.toml";
