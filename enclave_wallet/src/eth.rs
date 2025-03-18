use crate::derive::derive_key_from_path;
use crate::EnclaveKeyPair;
use crate::EnclaveWalletError;
use anyhow::anyhow;
use ethers_core::k256::ecdsa::SigningKey;
use ethers_signers::Signer as EthersSigner;
use ethers_signers::Wallet;

pub fn derive_eth_keypair(seed: &[u8], path: &str) -> Result<EnclaveKeyPair, EnclaveWalletError> {
    let private_key = derive_key_from_path(seed, path)?;
    let signing_key = match SigningKey::from_slice(&private_key) {
        Ok(key) => key,
        Err(err) => {
            return Err(EnclaveWalletError::AnyhowError(anyhow!(
                "ETH signing key error: {}",
                err
            )))
        }
    };
    let wallet = Wallet::from(signing_key);
    let public_address = wallet.address().as_bytes().to_vec();

    Ok(EnclaveKeyPair {
        private_key: private_key.to_vec(),
        public_address,
    })
}
