use enclave_agent::EnclaveAgentError;
use enclave_attestation::EnclaveAttestationError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum EnclaveTesterError {
    #[error("Enclave response error")]
    EnclaveResponseError,

    #[error("Invalid PCR0 error")]
    InvalidPCR0Error,

    #[error(transparent)]
    LogError(#[from] log::ParseLevelError),

    #[error(transparent)]
    EnclaveAgentError(#[from] EnclaveAgentError),

    #[error(transparent)]
    EnclaveAttestationError(#[from] EnclaveAttestationError),

    #[error(transparent)]
    DecodeError(#[from] prost::DecodeError),
}
