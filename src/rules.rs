use serde::Deserialize;
use std::{collections::HashMap, fs, path::Path};

use khronika::warn;

use crate::{RuleLevel, errors::Error};

pub(crate) mod condition;
pub(crate) mod filter;
pub(crate) mod matcher;

/// Rules with an invalid `condition` (referencing an undefined filter) are discarded at parse time with a warning.
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

/// Files that cannot be read, parsed, or validated are skipped with a warning.
/// Returns [`Error`] only for I/O failures on the directory itself.
pub fn parse_rules(path: &Path) -> Result<Vec<Rule>, Error> {
    let mut rules = Vec::new();

    for file in fs::read_dir(path)?.flatten() {
        let Ok(rdr) =
            fs::File::open(file.path()).inspect_err(|e| warn!("{}, {e}", file.path().display()))
        else {
            continue;
        };

        let Ok(r) = serde_yaml::from_reader::<_, Rule>(rdr)
            .map_err(Error::InvalidFormat)
            .inspect_err(|e| warn!("{}, {e}", file.path().display()))
        else {
            continue;
        };

        let filters = r.filters.keys().cloned().collect::<Vec<_>>();
        if let Err(err) = r.condition.validate(&filters) {
            warn!("Invalid condition on {}: {}", file.path().display(), err);
            continue;
        }

        rules.push(r);
    }

    Ok(rules)
}

#[cfg(test)]
mod tests;
