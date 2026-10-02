use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Website {
    url: String,
}

#[derive(Serialize, Deserialize)]
pub struct Websites {
    websites: Vec<Website>,
}
