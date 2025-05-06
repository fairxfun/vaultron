use crate::{generate_eth_public_pair, BlockChainPublicKeyPair, EnclaveCryptoEciesError, EnclaveCryptoError};
use anyhow::Result;
use typed_builder::TypedBuilder;

#[derive(Debug, Clone, TypedBuilder)]
pub struct EciesKeyPair {
    public_key: Vec<u8>,
    private_key: Vec<u8>,
}

impl From<BlockChainPublicKeyPair> for EciesKeyPair {
    fn from(value: BlockChainPublicKeyPair) -> Self {
        Self::builder()
            .public_key(value.public_key)
            .private_key(value.private_key)
            .build()
    }
}

impl EciesKeyPair {
    pub fn from_seed(seed: &[u8]) -> Result<Self, EnclaveCryptoError> {
        let key_pair = generate_eth_public_pair(seed)?;
        Ok(key_pair.into())
    }

    pub fn from_public_key(public_key: Vec<u8>) -> Result<Self, EnclaveCryptoError> {
        let key_pair = Self::builder().public_key(public_key).private_key(vec![]).build();
        Ok(key_pair)
    }

    pub fn public_key(&self) -> &[u8] {
        &self.public_key
    }

    pub fn encrypt_by_public_key(&self, data: &[u8]) -> Result<Vec<u8>, EnclaveCryptoError> {
        let encrypted_data =
            ecies::encrypt(&self.public_key, data).map_err(|_| EnclaveCryptoEciesError::EciesEncryptionError)?;
        Ok(encrypted_data)
    }

    pub fn decrypt_by_private_key(&self, data: &[u8]) -> Result<Vec<u8>, EnclaveCryptoError> {
        if self.private_key.is_empty() {
            return Err(EnclaveCryptoEciesError::EciesPrivateKeyError.into());
        }

        let decrypted_data =
            ecies::decrypt(&self.private_key, data).map_err(|_| EnclaveCryptoEciesError::EciesDecryptionError)?;
        Ok(decrypted_data)
    }
}
