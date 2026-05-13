use serde::{Deserialize, Deserializer};
use std::collections::HashMap;

use khronika::warn;

/// Comparison operator applied to a field value.
///
/// Encoded in YAML keys as `field|operator` (e.g. `process_name|contains`).
/// When no operator is specified, [`Exact`](FilterTypes::Exact) is used by default.
#[derive(Debug, Deserialize)]
pub enum FilterTypes {
    Exact,
    Contains,
    Startswith,
    Endswith,
    Gt,
    Gte,
    Lt,
    Lte,
}

fn match_filter(s: &str) -> Result<(String, FilterTypes), String> {
    match s.split_once('|') {
        Some((field, "exact")) => Ok((field.to_string(), FilterTypes::Exact)),
        Some((field, "contains")) => Ok((field.to_string(), FilterTypes::Contains)),
        Some((field, "startswith")) => Ok((field.to_string(), FilterTypes::Startswith)),
        Some((field, "endswith")) => Ok((field.to_string(), FilterTypes::Endswith)),
        Some((field, "gt")) => Ok((field.to_string(), FilterTypes::Gt)),
        Some((field, "gte")) => Ok((field.to_string(), FilterTypes::Gte)),
        Some((field, "lt")) => Ok((field.to_string(), FilterTypes::Lt)),
        Some((field, "lte")) => Ok((field.to_string(), FilterTypes::Lte)),
        Some((field, unknown)) => Err(format!("unknown filter {} for {}", unknown, field)),
        _ => Ok((s.to_string(), FilterTypes::Exact)),
    }
}

/// Typed scalar value used in filter comparisons.
///
/// All values within a single [`FieldFilter`] must share the same variant —
/// heterogeneous lists are rejected at parse time.
#[derive(Debug, Deserialize)]
pub enum Types {
    Boolean(bool),
    String(String),
    Integer(i64),
}

/// A single field comparison: multiple values are evaluated as a logical OR.
#[derive(Debug, Deserialize)]
pub struct FieldFilter {
    pub field: String,
    pub condition: FilterTypes,
    /// All values must share the same [`Types`] variant.
    pub values: Vec<Types>,
}

/// A named group of [`FieldFilter`]s deserialized from a YAML mapping.
///
/// Keys use the `field|operator` syntax. Entries with an unknown operator
/// or heterogeneous value types are dropped with a warning.
#[derive(Debug)]
pub struct Filters(pub Vec<FieldFilter>);

impl<'de> Deserialize<'de> for Filters {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let raw: HashMap<String, serde_yaml::Value> = HashMap::deserialize(d)?;

        let filters = raw
            .into_iter()
            .filter_map(|(k, v)| {
                let (field, condition) = match_filter(&k).inspect_err(|err| warn!(err)).ok()?;

                let value: Vec<Types> = match v {
                    serde_yaml::Value::Sequence(seq) => seq,
                    scalar => vec![scalar],
                }
                .into_iter()
                .filter_map(|v| {
                    serde_yaml::from_value(v)
                        .inspect_err(|err| warn!("{}", err))
                        .ok()
                })
                .collect();

                Some(FieldFilter {
                    condition,
                    field,
                    values: value
                        .first()
                        .map(std::mem::discriminant)
                        .is_none_or(|d| value.iter().all(|v| std::mem::discriminant(v) == d))
                        .then_some(value)
                        .or_else(|| {
                            warn!("Heterogeneous types values");
                            None
                        })?,
                })
            })
            .collect::<Vec<FieldFilter>>();

        Ok(Filters(filters))
    }
}
