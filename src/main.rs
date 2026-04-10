use std::path::Path;

use korelator::{
    InternalResult, load_configuration, quickwit::QuickwitClient, rules, telemetry::intialize,
};

#[tokio::main]
async fn main() -> InternalResult<()> {
    let configuration = load_configuration();
    intialize(configuration.log);
    tracing::info!("Korelator successfully initiated");

    // Test connection with simple request
    let client = QuickwitClient::new(configuration.quickwit_url.as_str());
    let result = client.search("security-events", "level:HIGH", 20).await?;
    println!(
        "{} hits for {}",
        result.num_hits, result.elapsed_time_micros
    );

    // Get pool of rules
    let rules_path = Path::new(&configuration.rules_path);
    let parsed_rules = rules::parser::parse_rules(rules_path).unwrap_or_default();

    dbg!(parsed_rules.len());

    Ok(())
}
