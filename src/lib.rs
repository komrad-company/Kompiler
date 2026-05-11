#![forbid(unsafe_code)]

use serde::Deserialize;
use std::fmt;

pub(crate) mod errors;
pub(crate) mod rules;

pub use errors::Error;
pub use rules::Rule;
pub use rules::condition::Condition;
pub use rules::filter::{FieldFilter, FilterTypes, Filters, Types};
pub use rules::matcher::{AggregationType, Matcher};
pub use rules::parse_rules;

/// Deserialized from lowercase YAML values (`informational`, `low`, `medium`, `high`, `critical`).
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RuleLevel {
    Informational,
    Low,
    Medium,
    High,
    Critical,
}

impl fmt::Display for RuleLevel {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let level = match self {
            RuleLevel::Informational => "informational",
            RuleLevel::Low => "low",
            RuleLevel::Medium => "medium",
            RuleLevel::High => "high",
            RuleLevel::Critical => "critical",
        };
        formatter.write_str(level)
    }
}
