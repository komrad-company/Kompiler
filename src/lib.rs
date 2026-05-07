use serde::Deserialize;

pub(crate) mod errors;
pub(crate) mod rules;

pub use errors::UnforgivableErrors;
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
