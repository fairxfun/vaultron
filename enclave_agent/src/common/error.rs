use enclave_protos::vaultron::common::v1::EnclaveAgentError as EnclaveAgentProtoError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum EnclaveAgentError {
    #[error("Enclave proto error")]
    EnclaveProtoError,

    #[error("Invalid request error")]
    InvalidRequestError,

    #[error("Cluster already initialized error")]
    ClusterAlreadyInitializedError,

    #[error("Enclave service tag empty error")]
    EnclaveServiceTagEmptyError,

    #[error("Enclave info not found error")]
    EnclaveInfoNotFoundError,

    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),

    #[error(transparent)]
    ProtobufEncodeError(#[from] prost::EncodeError),

    #[error(transparent)]
    VsockClientError(#[from] enclave_vsock::VsockClientError),

    #[error(transparent)]
    ProtobufDecodeError(#[from] prost::DecodeError),

    #[error(transparent)]
    EnclaveAgentControllerError(#[from] EnclaveAgentControllerError),

    #[error(transparent)]
    LogError(#[from] log::ParseLevelError),

    #[error(transparent)]
    IoError(#[from] std::io::Error),

    #[error(transparent)]
    ServiceDiscoveryError(#[from] service_discovery::VaultronServiceDiscoveryError),

    #[error(transparent)]
    HexError(#[from] hex::FromHexError),

    #[error(transparent)]
    AttestationError(#[from] enclave_attestation::EnclaveAttestationError),

    #[error("enclave grpc code {0} error")]
    EnclaveGrpcError(i32),

    #[error(transparent)]
    AnyError(#[from] anyhow::Error),
}

#[derive(Error, Debug)]
pub enum EnclaveAgentControllerError {
    #[error("Failed to start enclave: {0}")]
    StartEnclaveError(String),

    #[error("Failed to stop enclave: {0}")]
    StopEnclaveError(String),

    #[error("Failed to describe enclave: {0}")]
    DescribeEnclaveError(String),

    #[error(transparent)]
    IoError(#[from] std::io::Error),

    #[error(transparent)]
    SerdeJsonError(#[from] serde_json::Error),
}

impl From<EnclaveAgentControllerError> for EnclaveAgentProtoError {
    fn from(value: EnclaveAgentControllerError) -> Self {
        match value {
            EnclaveAgentControllerError::StartEnclaveError(_) => EnclaveAgentProtoError::StartEnclaveError,
            EnclaveAgentControllerError::StopEnclaveError(_) => EnclaveAgentProtoError::StopEnclaveError,
            EnclaveAgentControllerError::DescribeEnclaveError(_) => EnclaveAgentProtoError::DescribeEnclaveError,
            EnclaveAgentControllerError::IoError(_) => EnclaveAgentProtoError::IoError,
            EnclaveAgentControllerError::SerdeJsonError(_) => EnclaveAgentProtoError::SerdeJsonError,
        }
    }
}
