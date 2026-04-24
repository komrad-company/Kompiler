use std::{env, error::Error, fs::File, io::BufReader};

use serde::Deserialize;
use serde_json::from_reader;

use crate::errors::UnforgivableErrors;

pub mod configuration;
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

pub fn load_configuration() -> Result<configuration::Configuration, UnforgivableErrors> {
    let configuration_path: String =
        env::var("CONFIGURATION_PATH").unwrap_or_else(|_| "configuration.json".to_string());

    let file = File::open(&configuration_path).map_err(|_| {
        UnforgivableErrors::MissingConfigurationFile {
            path: configuration_path,
        }
    })?;

    let reader = BufReader::new(file);
    let conf = from_reader(reader).map_err(UnforgivableErrors::InvalidFormat)?;

    Ok(conf)
}
