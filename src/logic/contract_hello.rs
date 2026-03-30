use crate::logic::network::{get_stored_network_id, NetworkId};
// ===========================================
// hello_contract_id_for_network
// GREETING_CONTRACT_METHODS
// ===========================================
const HELLO_TESTNET_CONTRACT_ID: &str = "hello.sleet.testnet";
const HELLO_MAINNET_CONTRACT_ID: &str = "hello.sleet.near";
// ===========================================
/// Get the hello contract ID based on the current network
pub fn hello_contract_id_for_network() -> &'static str {
    let network_id = get_stored_network_id();
    match network_id {
        NetworkId::Testnet => HELLO_TESTNET_CONTRACT_ID,
        NetworkId::Mainnet => HELLO_MAINNET_CONTRACT_ID,
    }
}
// ===========================================
// Contract methods
pub const GREETING_CONTRACT_METHODS: GreetingContractMethods = GreetingContractMethods {
    get_greeting: "get_greeting",
    set_greeting: "set_greeting",
};

#[derive(Debug, Clone, Copy)]
pub struct GreetingContractMethods {
    pub get_greeting: &'static str,
    pub set_greeting: &'static str,
}
// ===========================================
// copyright 2026 by sleet.near
