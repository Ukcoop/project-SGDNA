use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub promoters: Vec<String>,
    pub terminators: Vec<String>,
    pub rbs: Vec<String>,
    pub rep_origins: Vec<String>,
}
