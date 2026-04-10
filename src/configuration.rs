use std::path::PathBuf;

use serde::Deserialize;

use crate::LogLevel;

#[derive(Deserialize)]
pub struct TelemetryConfiguration {
    pub level: LogLevel,
    pub file: Option<PathBuf>,
}
#[derive(Deserialize)]
pub struct Configuration {
    pub quickwit_url: String,
    pub rules_path: String,
    pub log: TelemetryConfiguration,
}
