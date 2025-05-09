use super::ENCRYPTION_KEY_LENGTH;
use crate::{EnclaveCryptoAesError, EnclaveCryptoError};
use aes_gcm::{aead::Aead, Aes256Gcm, KeyInit, Nonce};
use typed_builder::TypedBuilder;

pub const AES_GCM_NONCE_LENGTH: usize = 12;

pub struct AesGcmKey {
    key: Aes256Gcm,
}

impl AesGcmKey {
    pub fn new(seed: &[u8]) -> Result<Self, EnclaveCryptoError> {
        if seed.len() != ENCRYPTION_KEY_LENGTH {
            return Err(EnclaveCryptoAesError::InvalidKeyLengthError.into());
        }

        let key = Aes256Gcm::new_from_slice(seed).map_err(|_| EnclaveCryptoAesError::InvalidKeyLengthError)?;
        Ok(Self { key })
    }

    pub fn encrypt(&self, nonce: &[u8], data: &[u8]) -> Result<Vec<u8>, EnclaveCryptoError> {
        if nonce.len() != AES_GCM_NONCE_LENGTH {
            return Err(EnclaveCryptoAesError::InvalidAesGcmNonceLengthError.into());
        }

        let nonce = Nonce::from_slice(nonce);
        let ciphertext = self
            .key
            .encrypt(nonce, data)
            .map_err(|_| EnclaveCryptoAesError::AesGcmEncryptionError)?;
        let aes_gcm_data = AesGcmData::builder()
            .nonce(nonce.to_vec())
            .encrypted_data(ciphertext)
            .build();
        Ok(aes_gcm_data.serialize())
    }

    pub fn decrypt(&self, data: &[u8]) -> Result<Vec<u8>, EnclaveCryptoError> {
        let aes_gcm_data = AesGcmData::deserialize(data)?;
        let nonce = Nonce::from_slice(&aes_gcm_data.nonce);
        let plaintext = self
            .key
            .decrypt(nonce, aes_gcm_data.encrypted_data.as_slice())
            .map_err(|_| EnclaveCryptoAesError::AesGcmDecryptionError)?;
        Ok(plaintext)
    }
}

#[derive(Debug, Clone, TypedBuilder)]
pub struct AesGcmData {
    pub nonce: Vec<u8>,
    pub encrypted_data: Vec<u8>,
}

impl AesGcmData {
    pub fn serialize(&self) -> Vec<u8> {
        let mut data = Vec::new();
        data.extend_from_slice(&self.nonce);
        data.extend_from_slice(&self.encrypted_data);
        data
    }

    pub fn deserialize(data: &[u8]) -> Result<Self, EnclaveCryptoError> {
        if data.len() < AES_GCM_NONCE_LENGTH {
            return Err(EnclaveCryptoAesError::InvalidAesGcmDataLengthError.into());
        }

        let (nonce, encrypted_data) = data.split_at(AES_GCM_NONCE_LENGTH);
        Ok(Self::builder()
            .nonce(nonce.to_vec())
            .encrypted_data(encrypted_data.to_vec())
            .build())
    }
}
