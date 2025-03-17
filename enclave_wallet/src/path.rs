pub fn get_eth_derivation_path(index: Option<u32>) -> String {
    format!("m/44'/60'/0'/0/{}", index.unwrap_or(0))
}

pub fn get_solana_derivation_path(index: Option<u32>) -> String {
    format!("m/44'/501'/{}'/0'", index.unwrap_or(0))
}

// Scheme ED25519 https://docs.sui.io/concepts/cryptography/transaction-auth/keys-addresses
pub fn get_sui_derivation_path(index: Option<u32>) -> String {
    format!("m/44'/784'/{}'/0'/0'", index.unwrap_or(0))
}
