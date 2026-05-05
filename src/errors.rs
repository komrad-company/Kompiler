use thiserror::Error;

#[derive(Debug, Error)]
pub enum UnforgivableErrors {
    #[error("Invalid file format {0}")]
    InvalidFormat(#[from] serde_yaml::Error),
    #[error("File not found {path}")]
    MissingConfigurationFile { path: String },
    #[error("Invalid rules path {0}")]
    RulesPathNotFound(#[from] std::io::Error),
}
