use super::derive_key_from_path;
use super::get_eth_derivation_path;
use super::BlockChainPublicKeyPair;
use crate::BlockChainKeyPair;
use crate::EnclaveCryptoChainKeyError;
use crate::EnclaveCryptoError;
use ethers_core::k256::ecdsa::SigningKey;
use ethers_signers::Signer as EthersSigner;
use ethers_signers::Wallet;

pub fn derive_eth_keypair(seed: &[u8], index: Option<u32>) -> Result<BlockChainKeyPair, EnclaveCryptoError> {
    let (private_key, signing_key) = derive_eth_sining_key(seed, index)?;
    let wallet = Wallet::from(signing_key);
    let public_address = wallet.address().as_bytes().to_vec();
    Ok(BlockChainKeyPair::builder()
        .private_key(private_key)
        .public_address(public_address)
        .build())
}

pub fn generate_eth_public_pair(seed: &[u8]) -> Result<BlockChainPublicKeyPair, EnclaveCryptoError> {
    let (private_key, signing_key) = derive_eth_sining_key(seed, None)?;
    let verifying_key = signing_key.verifying_key();
    let public_key = verifying_key.to_encoded_point(false).as_bytes().to_vec();
    Ok(BlockChainPublicKeyPair::builder()
        .private_key(private_key)
        .public_key(public_key)
        .build())
}

fn derive_eth_sining_key(seed: &[u8], index: Option<u32>) -> Result<(Vec<u8>, SigningKey), EnclaveCryptoError> {
    let path = get_eth_derivation_path(index);
    let private_key = derive_key_from_path(seed, &path)?;
    let signing_key = match SigningKey::from_slice(&private_key) {
        Ok(key) => key,
        Err(_) => return Err(EnclaveCryptoChainKeyError::SigningKeyError.into()),
    };
    Ok((private_key, signing_key))
}
