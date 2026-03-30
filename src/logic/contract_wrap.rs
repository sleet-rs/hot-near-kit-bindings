use crate::logic::network::{get_stored_network_id, NetworkId};
// ===========================================
// wrap_contract_id_for_network
// WRAP_NEAR_METHODS
// ===========================================
const WRAP_TESTNET_CONTRACT_ID: &str = "wrap.testnet";
const WRAP_MAINNET_CONTRACT_ID: &str = "wrap.near";
// ===========================================
/// Get the wrap contract ID based on the current network
pub fn wrap_contract_id_for_network() -> &'static str {
    let network_id = get_stored_network_id();
    match network_id {
        NetworkId::Testnet => WRAP_TESTNET_CONTRACT_ID,
        NetworkId::Mainnet => WRAP_MAINNET_CONTRACT_ID,
    }
}
// ===========================================
// Wrap Near contract methods
pub const WRAP_NEAR_METHODS: WrapNearMethods = WrapNearMethods {
    near_deposit: "near_deposit",
    near_withdraw: "near_withdraw",
};

#[derive(Debug, Clone, Copy)]
pub struct WrapNearMethods {
    pub near_deposit: &'static str,
    pub near_withdraw: &'static str,
}
// ===========================================
// copyright 2026 by sleet.near
