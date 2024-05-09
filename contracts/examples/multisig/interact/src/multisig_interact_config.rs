use multiversx_sc_scenario::imports::Bech32Address;
use serde::Deserialize;
use std::io::Read;

/// Config file
const CONFIG_FILE: &str = "config.toml";

/// Multisig Interact configuration
#[derive(Debug, Deserialize)]
pub struct Config {
    pub gateway: String,
    pub quorum: usize,
    pub wegld_address: Bech32Address,
    pub wegld_token_identifier: String,
}

impl Config {
    // Deserializes config from file
    pub fn load_config() -> Self {
        let mut file = std::fs::File::open(CONFIG_FILE).unwrap();
        let mut content = String::new();
        file.read_to_string(&mut content).unwrap();
        toml::from_str(&content).unwrap()
    }

    // // Returns the gateway
    // pub fn gateway(&self) -> &str {
    //     &self.gateway
    // }

    // Returns the quorum
    // pub fn quorum(&self) -> usize {
    //     self.quorum
    // }

    // pub fn wegld_address(&self) -> Bech32Address {
    //     Bech32Address::from_bech32_string(bech32)
    // }
}
