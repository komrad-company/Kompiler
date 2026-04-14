use serde::Deserialize;

use crate::RuleLevel;

pub mod parser;

// Matching and aggregations structs
#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "type")]
pub enum AggregationType {
    Count { threshold: u16 },
}
#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "type")]
pub enum MatchType {
    Single, // Stateless, immediate match
    Threshold {
        timeframe_secs: u32,
        aggregate: AggregationType,
        group_by: Vec<String>,
    },
}

#[derive(Debug, Clone, Deserialize)]
pub struct Rule {
    pub id: String,
    pub title: String,
    pub level: RuleLevel,
    pub description: Option<String>,
    pub tags: Option<Vec<String>>,
    pub match_type: MatchType,
    // pub detection: Detection,
}

#[cfg(test)]
mod tests;
