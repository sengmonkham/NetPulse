use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentConfig {
    pub network: NetworkConfig,
    pub measurements: MeasurementConfig,
    pub privacy: PrivacyConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkConfig {
    pub listen_addresses: Vec<String>,
    pub bootstrap_peers: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeasurementConfig {
    pub interval: u64, // seconds
    pub targets: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrivacyConfig {
    pub epsilon: f64,
    pub enable_geolocation_fuzzing: bool,
}

impl AgentConfig {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let contents = fs::read_to_string(path)?;
        let config: AgentConfig = toml::from_str(&contents)?;
        Ok(config)
    }
}
