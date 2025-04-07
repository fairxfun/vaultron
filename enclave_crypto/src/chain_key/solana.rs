use super::derive_key_from_path;
use super::get_solana_derivation_path;
use crate::BlockChainKeyPair;
use crate::EnclaveCryptoChainKeyError;
use crate::EnclaveCryptoError;
use solana_keypair::keypair_from_seed;
use solana_signer::Signer;

pub fn derive_solana_keypair(seed: &[u8], index: Option<u32>) -> Result<BlockChainKeyPair, EnclaveCryptoError> {
    let path = get_solana_derivation_path(index);
    let seed_arr = derive_key_from_path(seed, &path)?;
    let solana_keypair =
        keypair_from_seed(&seed_arr).map_err(|_| EnclaveCryptoChainKeyError::DeriveKeyFromPathError)?;
    let base58_address = solana_keypair.pubkey().to_string().into_bytes();
    let private_key_full = solana_keypair.to_bytes().to_vec();

    Ok(BlockChainKeyPair {
        private_key: private_key_full,
        public_address: base58_address,
    })
}
