use std::path::Path;

use korelator::{load_configuration, rules::parser, telemetry::intialize};

#[tokio::main]
async fn main() {
    let configuration = load_configuration().unwrap_or_else(|err| {
        eprintln!("Fatal Error: {err}");
        std::process::exit(1)
    });

    intialize(configuration.log);
    tracing::debug!("Korelator successfully initiated");

    // Get pool of rules
    let rules_path = Path::new(&configuration.rules_path);
    let parsed_rules = parser::parse_rules(rules_path)
        .map_err(|e| {
            tracing::error!("Unforgivable error {e}");
            std::process::exit(2)
        })
        .unwrap();

    dbg!(parsed_rules.len());
}
