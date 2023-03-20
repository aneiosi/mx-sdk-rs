use forwarder_queue::QueuedCallType;
use multiversx_sc_snippets::{multiversx_sc::types::{EgldOrEsdtTokenIdentifier, TokenIdentifier}, multiversx_sc_scenario::DebugApi};
use serde::Deserialize;
use std::io::Read;

/// Config file
const CONFIG_FILE: &str = "config.toml";

/// Multisig Interact configuration
#[derive(Debug, Deserialize)]
pub struct Config {
    gateway: String,
    call_type: String,
    token_id: String,
    token_nonce: u64,
    amount: u64,
}

impl Config {
    // Deserializes config from file
    pub fn load_config() -> Self {
        let mut file = std::fs::File::open(CONFIG_FILE).unwrap();
        let mut content = String::new();
        file.read_to_string(&mut content).unwrap();
        toml::from_str(&content).unwrap()
    }

    // Returns the gateway
    pub fn gateway(&self) -> &str {
        &self.gateway
    }

    pub fn call_type(&self) -> QueuedCallType {
        match self.call_type.as_str() {
            "Sync" => QueuedCallType::Sync,
            "LegacyAsync" => QueuedCallType::LegacyAsync,
            "TransferExecute" => QueuedCallType::TransferExecute,
            &_ => todo!(),
        }
    }

    pub fn token_id(&self) -> EgldOrEsdtTokenIdentifier<DebugApi> {
        match self.token_id.as_str() {
            "EGLD" => EgldOrEsdtTokenIdentifier::egld(),
            _ => EgldOrEsdtTokenIdentifier::esdt(TokenIdentifier::from(self.token_id.as_str())),
        }
    }

    pub fn token_amount(&self) -> u64 {
        self.amount
    }

    pub fn token_nonce(&self) -> u64 {
        self.token_nonce
    }
}
