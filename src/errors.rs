use thiserror::Error;

/// Fatal errors that prevent Kompiler from operating.
///
/// These are non-recoverable and should cause the caller to abort startup.
#[derive(Debug, Error)]
pub enum UnforgivableErrors {
    #[error("Invalid file format {0}")]
    InvalidFormat(#[from] serde_yaml::Error),
    #[error("Invalid rules path {0}")]
    RulesPathNotFound(#[from] std::io::Error),
}
