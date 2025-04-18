use enclave_attestation::EnclaveAttestationError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum EnclaveTesterError {
    #[error("Enclave response error")]
    EnclaveResponseError,

    #[error("Enclave agent response error")]
    EnclaveAgentResponseError,

    #[error("Invalid PCR0 error")]
    InvalidPCR0Error,

    #[error("Invalid response error")]
    InvalidResponseError,

    #[error("grpc error with code {0}")]
    GrpcError(tonic::Code),

    #[error(transparent)]
    LogError(#[from] log::ParseLevelError),

    #[error(transparent)]
    EnclaveAttestationError(#[from] EnclaveAttestationError),

    #[error(transparent)]
    DecodeError(#[from] prost::DecodeError),
}

impl From<tonic::Status> for EnclaveTesterError {
    fn from(status: tonic::Status) -> Self {
        EnclaveTesterError::GrpcError(status.code())
    }
}
