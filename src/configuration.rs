use std::path::PathBuf;

use serde::Deserialize;

use crate::RuleLevel;

#[derive(Deserialize)]
#[serde(untagged)]
pub enum TelemetryOutput {
    Both { file: PathBuf, telemetry: String },
    File { file: PathBuf },
    Remote { telemetry: String },
}

#[derive(Deserialize)]
pub struct TelemetryConfiguration {
    pub level: RuleLevel,
    #[serde(flatten)]
    pub output: TelemetryOutput,
}
#[derive(Deserialize)]
pub struct Configuration {
    pub quickwit_url: String,
    pub rules_path: String,
    pub log: TelemetryConfiguration,
}
