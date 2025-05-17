use thiserror::Error;

/// Error types for angler operations
#[derive(Error, Debug)]
pub enum AnglerError {
    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Git error: {0}")]
    Git(String),

    #[error("Hook error: {0}")]
    Hook(String),

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
}
