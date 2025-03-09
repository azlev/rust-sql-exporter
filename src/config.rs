use std::fs;

use serde::{Deserialize, Serialize};

use crate::metric::MetricType;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Query {
    pub query: String,
    pub metric: String,
    #[serde(rename = "type")]
    pub type_: MetricType,
    pub help: String,
    pub interval: Option<u64>,
}

// wrapper function to isolate fs read
pub fn loadconfig(filename: String) -> Vec<Query> {
    let config: String = fs::read_to_string(filename).unwrap();
    serde_yaml::from_str(&config).unwrap()
}
