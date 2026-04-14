use std::{fs, path::Path};

use crate::errors::UnforgivableErrors;

use super::Rule;

// functions
pub fn parse_rules(path: &Path) -> Result<Vec<Rule>, UnforgivableErrors> {
    let rules = fs::read_dir(path)?
        .flatten()
        .filter_map(|file| {
            fs::File::open(file.path())
                .map_err(|e| tracing::warn!("{}, {e}", file.path().display()))
                .ok()
                .and_then(|rdr| {
                    serde_yaml::from_reader(rdr)
                        .map_err(|err| tracing::warn!("{}, {err}", file.path().display()))
                        .ok()
                })
        })
        .collect();

    Ok(rules)
}
