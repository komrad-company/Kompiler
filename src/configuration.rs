use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Configuration {
    pub quickwit_url: String
}
