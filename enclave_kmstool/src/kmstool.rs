use typed_builder::TypedBuilder;

use crate::EnclaveKmstoolError;

#[derive(Debug, Default, TypedBuilder)]
pub struct KmsConfigure {
    pub enable_logging: bool,
    pub proxy_port: u32,
    pub aws_region: String,
    pub aws_access_key_id: String,
    pub aws_secret_access_key: String,
    pub aws_session_token: String,
    pub kms_key_id: String,
    pub kms_encryption_algorithm: String,
}

#[derive(Debug, Clone, TypedBuilder)]
pub struct KmsInitRequest {
    pub enable_logging: bool,
    pub proxy_port: u32,
    pub aws_region: String,
    pub aws_access_key_id: String,
    pub aws_secret_access_key: String,
    pub aws_session_token: String,
    pub kms_key_id: String,
    pub kms_encryption_algorithm: String,
}

#[derive(Debug, Clone, TypedBuilder)]
pub struct KmsEncryptRequest {
    pub plaintext: Vec<u8>,
}

#[derive(Debug, TypedBuilder)]
pub struct KmsEncryptResponse {
    pub ciphertext: Vec<u8>,
}

#[derive(Debug, Clone, TypedBuilder)]
pub struct KmsDecryptRequest {
    pub ciphertext: Vec<u8>,
}

#[derive(Debug, TypedBuilder)]
pub struct KmsDecryptResponse {
    pub plaintext: Vec<u8>,
}

pub trait KmsToolTrait {
    fn init(&self, request: KmsInitRequest) -> anyhow::Result<(), EnclaveKmstoolError>;
    fn encrypt(&self, request: KmsEncryptRequest) -> anyhow::Result<KmsEncryptResponse, EnclaveKmstoolError>;
    fn decrypt(&self, request: KmsDecryptRequest) -> anyhow::Result<KmsDecryptResponse, EnclaveKmstoolError>;
}
