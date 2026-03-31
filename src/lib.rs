use std::{
    env,
    fmt::Display,
    fs::File,
    io::BufReader
};

use serde_json::from_reader;

pub mod configuration;
pub mod quickwit;

pub fn the_unforgivable_error<E: Display>(error: E, code: i32) -> ! {
    println!("Unforgivable: {}", error);
    std::process::exit(code)
}

pub fn load_configuration() -> configuration::Configuration {
    let configuration_path: String = env::var("CONFIGURATION_PATH")
        .unwrap_or_else(|_| "configuration.json".to_string());


    let file = File::open(&configuration_path).unwrap_or_else(
        |err| the_unforgivable_error(err, 1)
    );

    let reader = BufReader::new(file);
    from_reader(reader).unwrap_or_else(
        |err| the_unforgivable_error(err, 1)
    )
}