use super::EnclaveError;
use enclave_crypto::EciesKeyPair;
use enclave_vsock::DEFAULT_ENCLAVE_VSOCK_PORT;
use typed_builder::TypedBuilder;

pub const GIT_REVISION: &str = env!("VAULTRON_GIT_REVISION");

#[derive(Debug, Clone, TypedBuilder)]
pub struct EnclaveSettings {
    pub log_level: String,
    pub port: u32,
    pub pcr0: Vec<u8>,
    pub local_key: EciesKeyPair,
}

impl EnclaveSettings {
    pub fn new(pcr0: Vec<u8>, seed: Vec<u8>) -> Result<Self, EnclaveError> {
        let local_key = EciesKeyPair::from_seed(&seed)?;
        Ok(Self::builder()
            .pcr0(pcr0)
            .log_level("info".to_string())
            .port(DEFAULT_ENCLAVE_VSOCK_PORT)
            .local_key(local_key)
            .build())
    }
}
