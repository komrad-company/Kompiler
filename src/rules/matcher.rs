use serde::Deserialize;

// Matching and aggregations structs
#[derive(Debug, Clone, Deserialize)]
pub enum AggregationType {
    Count(u16),
}
#[derive(Debug, Clone, Deserialize)]
pub enum Matcher {
    Single,
    Threshold {
        timeframe_secs: u32,
        aggregate: AggregationType,
        group_by: Vec<String>,
    },
}
