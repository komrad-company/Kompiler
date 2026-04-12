use thiserror::Error;

#[derive(Debug, Error)]
pub enum UnforgivableErrors {
    #[error("Invalid file format {0}")]
    InvalidFormat(#[from] serde_json::Error),
    #[error("File not found {path}")]
    MissingConfigurationFile { path: String },
}

#[derive(Debug, Error)]
pub enum AppError {
    #[error("[ConfigurationError] {0}")]
    ConfigurationError(#[from] UnforgivableErrors),
}
