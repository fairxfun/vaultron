use crate::derive::derive_key_from_path;
use crate::EnclaveWalletError;
use crate::KeyPair;
use anyhow::anyhow;
use solana_sdk::signature::Keypair as SolanaKeypair;
use solana_sdk::signature::SeedDerivable;
use solana_sdk::signature::Signer;

pub fn derive_solana_keypair(seed: &[u8], path: &str) -> Result<KeyPair, EnclaveWalletError> {
    let seed_arr = derive_key_from_path(seed, path)?;
    let solana_keypair = SolanaKeypair::from_seed(&seed_arr)
        .map_err(|e| EnclaveWalletError::AnyhowError(anyhow!("Solana keypair error: {}", e)))?;

    let base58_address = solana_keypair.pubkey().to_string().into_bytes();
    let private_key_full = solana_keypair.to_bytes().to_vec();

    Ok(KeyPair {
        private_key: private_key_full,
        public_address: base58_address,
    })
}
