use korelator::{
    load_configuration,
    quickwit::QuickwitClient
};


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let configuration = load_configuration();

    let client = QuickwitClient::new(configuration.quickwit_url.as_str());
    let result = client.search("security-events", "level:HIGH", 20).await?;
    println!("{} hits for {}", result.num_hits, result.elapsed_time_micros);

    Ok(())
}