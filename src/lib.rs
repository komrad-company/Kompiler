use std::error::Error;

use serde::Deserialize;

pub mod errors;
pub mod rules;

pub type InternalResult<T> = Result<T, Box<dyn Error>>;

// Global struct
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RuleLevel {
    Informational,
    Low,
    Medium,
    High,
    Critical,
}
