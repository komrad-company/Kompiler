use serde::Deserialize;
use std::{collections::HashMap, fs, path::Path};

use crate::{RuleLevel, errors::UnforgivableErrors};

pub mod condition;
pub mod filter;
pub mod matcher;

#[derive(Debug, Deserialize)]
pub struct Rule {
    pub id: String,
    pub title: String,
    pub level: RuleLevel,
    pub description: Option<String>,
    pub tags: Option<Vec<String>>,
    pub matcher: matcher::Matcher,
    pub filters: HashMap<String, Vec<filter::Filters>>,
    pub condition: condition::Condition,
}

pub fn parse_rules(path: &Path) -> Result<Vec<Rule>, UnforgivableErrors> {
    let rules = fs::read_dir(path)?
        .flatten()
        .filter_map(|file| {
            fs::File::open(file.path())
                .map_err(|e| tracing::error!("{}, {e}", file.path().display()))
                .ok()
                .and_then(|rdr| {
                    serde_yaml::from_reader(rdr)
                        .map_err(|err| tracing::error!("{}, {err}", file.path().display()))
                        .ok()
                        .and_then(|r: Rule| {
                            let filters = r.filters.keys().cloned().collect();
                            r.condition
                                .validate(&filters)
                                .map_err(|err| {
                                    tracing::error!(
                                        "Invalid condition on {}: {}",
                                        file.path().display(),
                                        err
                                    )
                                })
                                .ok()
                                .map(|_| r)
                        })
                })
        })
        .collect();

    Ok(rules)
}

#[cfg(test)]
mod tests;
