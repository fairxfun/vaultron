use crate::EnclaveError;
use anyhow::Result;
use enclave_crypto::{random_bytes_by_rng, AesGcmKey, EciesKeyPair, AES_GCM_NONCE_LENGTH};
use typed_builder::TypedBuilder;

#[derive(TypedBuilder)]
pub struct EnclaveClusterKeys {
    seed: Vec<u8>,
    ase_key: AesGcmKey,
    ecies_key: EciesKeyPair,
}

impl EnclaveClusterKeys {
    pub fn new(seed: &[u8]) -> Result<Self, EnclaveError> {
        let ase_key = AesGcmKey::new(seed)?;
        let ecies_key = EciesKeyPair::from_seed(seed)?;
        Ok(Self::builder()
            .seed(seed.to_vec())
            .ase_key(ase_key)
            .ecies_key(ecies_key)
            .build())
    }

    pub fn get_cluster_public_key(&self) -> Vec<u8> {
        self.ecies_key.public_key().to_vec()
    }

    pub fn encrypt_seed(&self, public_key: Vec<u8>) -> Result<Vec<u8>, EnclaveError> {
        let key = EciesKeyPair::from_public_key(public_key)?;
        let encrypted_seed = key.encrypt_by_public_key(&self.seed)?;
        Ok(encrypted_seed)
    }

    pub fn encrypt(&self, data: &[u8]) -> Result<Vec<u8>, EnclaveError> {
        let nonce = random_bytes_by_rng(AES_GCM_NONCE_LENGTH);
        let encrypted_data = self.ase_key.encrypt(&nonce, data)?;
        Ok(encrypted_data)
    }

    pub fn decrypt(&self, data: &[u8]) -> Result<Vec<u8>, EnclaveError> {
        let decrypted_data = self.ase_key.decrypt(data)?;
        Ok(decrypted_data)
    }

    pub fn encrypt_by_public_key(&self, data: &[u8]) -> Result<Vec<u8>, EnclaveError> {
        let encrypted_data = self.ecies_key.encrypt_by_public_key(data)?;
        Ok(encrypted_data)
    }

    pub fn decrypt_by_private_key(&self, data: &[u8]) -> Result<Vec<u8>, EnclaveError> {
        let decrypted_data = self.ecies_key.decrypt_by_private_key(data)?;
        Ok(decrypted_data)
    }
}
