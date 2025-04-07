use super::get_sui_derivation_path;
use crate::BlockChainKeyPair;
use crate::EnclaveCryptoChainKeyError;
use crate::EnclaveCryptoError;
use anyhow::Result;
use bip32::DerivationPath;
use std::str::FromStr;
use sui_keys::key_derive::derive_key_pair_from_path;
use sui_types::crypto::KeypairTraits;
use sui_types::crypto::SignatureScheme;
use sui_types::crypto::SuiKeyPair;
use sui_types::crypto::ToFromBytes;

pub fn derive_sui_keypair(seed: &[u8], index: Option<u32>) -> Result<BlockChainKeyPair, EnclaveCryptoError> {
    let path = get_sui_derivation_path(index);
    let derivation_path =
        DerivationPath::from_str(&path).map_err(|_| EnclaveCryptoChainKeyError::DeriveKeyPathError)?;
    let (sui_address, keypair) = derive_key_pair_from_path(seed, Some(derivation_path), &SignatureScheme::ED25519)
        .map_err(|_| EnclaveCryptoChainKeyError::DeriveKeyFromPathError)?;
    let private_key = match keypair {
        SuiKeyPair::Ed25519(ed25519_keypair) => ed25519_keypair.private(),
        _ => {
            return Err(EnclaveCryptoChainKeyError::WrongKeyPairError.into());
        }
    };
    Ok(BlockChainKeyPair {
        private_key: private_key.as_bytes().to_vec(),
        public_address: sui_address.to_vec(),
    })
}
