use crate::derive::derive_key_from_path;
use crate::EnclaveKeyPair;
use crate::EnclaveWalletError;
use anyhow::anyhow;
use solana_keypair::keypair_from_seed;
use solana_signer::Signer;

pub fn derive_solana_keypair(seed: &[u8], path: &str) -> Result<EnclaveKeyPair, EnclaveWalletError> {
    let seed_arr = derive_key_from_path(seed, path)?;
    let solana_keypair = keypair_from_seed(&seed_arr)
        .map_err(|e| EnclaveWalletError::AnyhowError(anyhow!("Solana keypair error: {}", e)))?;
    let base58_address = solana_keypair.pubkey().to_string().into_bytes();
    let private_key_full = solana_keypair.to_bytes().to_vec();

    Ok(EnclaveKeyPair {
        private_key: private_key_full,
        public_address: base58_address,
    })
}
