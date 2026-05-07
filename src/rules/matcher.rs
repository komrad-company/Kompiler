use serde::Deserialize;

/// Aggregation function applied over a time window in a [`Matcher::Threshold`] rule.
#[derive(Debug, Clone, Deserialize)]
pub enum AggregationType {
    Count(u16),
}

#[derive(Debug, Clone, Deserialize)]
pub enum Matcher {
    /// Triggers on every event that satisfies the condition.
    Single,
    /// Triggers when the aggregation threshold is reached within the time window.
    Threshold {
        timeframe_secs: u32,
        aggregate: AggregationType,
        /// Fields used to partition events before aggregating.
        group_by: Vec<String>,
    },
}
