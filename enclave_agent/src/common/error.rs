use thiserror::Error;

#[derive(Error, Debug)]
pub enum EnclaveAgentError {
    #[error("Failed to send request error: {0}")]
    VsockClientError(#[from] enclave_vsock::VsockClientError),

    #[error("Protocol buffer encoding error")]
    ProtobufEncodeError(#[from] prost::EncodeError),

    #[error("Invalid response error")]
    InvalidResponseError,

    #[error("Protocol buffer decoding error")]
    ProtobufDecodeError(#[from] prost::DecodeError),

    #[error("Enclave proto error")]
    EnclaveProtoError,

    #[error(transparent)]
    LogError(#[from] log::ParseLevelError),
}
