use crate::logic::network::{get_stored_network_id, NetworkId};
// ===========================================
// rhea_contract_id_for_network
// ===========================================
const REF_TESTNET_CONTRACT_ID: &str = "ref-finance-101.testnet";
const REF_MAINNET_CONTRACT_ID: &str = "v2.ref-finance.near";
// ===========================================
/// Get the rhea/ref-exchange contract ID based on the current network
pub fn rhea_contract_id_for_network() -> &'static str {
    let network_id = get_stored_network_id();
    match network_id {
        NetworkId::Testnet => REF_TESTNET_CONTRACT_ID,
        NetworkId::Mainnet => REF_MAINNET_CONTRACT_ID,
    }
}
// ===========================================
// copyright 2026 by sleet.near
