use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Feature {
    pub name: String,
    pub start: u32,
    pub end: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GBContainer {
    pub name: String,
    pub structure: String,
    pub features: Vec<Feature>,
    pub dna: String,
}
