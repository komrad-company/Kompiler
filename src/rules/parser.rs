use std::{fs, io::Error, path::Path};

use super::Rule;

// functions
pub fn parse_rules(path: &Path) -> Result<Vec<Rule>, Error> {
    let rules = fs::read_dir(path)?
        .flatten()
        .filter_map(|e| {
            fs::File::open(&e.path())
                .map_err(|open| eprintln!("Opening {}: {}", e.path().display(), open))
                .ok()
                .and_then(|file| {
                    serde_yaml::from_reader(file)
                        .map_err(|read| eprintln!("reading {}: {}", e.path().display(), read))
                        .ok()
                })
        })
        .collect();

    Ok(rules)
}
