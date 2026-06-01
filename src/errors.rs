use thiserror::Error;

pub type SFFResult<T> = Result<T, SFFError>;

#[derive(Debug, Error)]
pub enum SFFError {
    #[error("Sober configuration file not found. Please check the path or launch Sober once")]
    ConfigNotFound(#[source] std::io::Error),

    #[error("I/O error occurred while accessing the disk: {0}")]
    IOError(#[from] std::io::Error),

    #[error("JSON structure error: {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("Flag '{0}' was not found in the Sober configuration")]
    FlagNotFound(String),

    #[error("Failed to read environment variable: {0}")]
    EnvVarError(#[from] std::env::VarError),
}
