/// Library related error management
pub mod error;

/// Transpililing markdown to github flavoured markdown
pub mod gfm;

/// IO with for custom structs
pub mod fs;

/// Config file parser.
pub mod config;

/// Default configuration file.
pub const DEFAULT_CONFIG: &str = "smd/config.toml";
