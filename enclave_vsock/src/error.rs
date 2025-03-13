use thiserror::Error;

#[derive(Error, Debug)]
pub enum EnclaveVsockServerError {
    #[error("IO error")]
    IoError(#[from] std::io::Error),
}

#[derive(Error, Debug)]
pub enum EnclaveVsockClientError {
    #[error("Protocol buffer encoding error")]
    ProtobufEncodeError(#[from] prost::EncodeError),

    #[error("Protocol buffer decoding error")]
    ProtobufDecodeError(#[from] prost::DecodeError),

    #[error("Lock error")]
    LockError,

    #[error("IO error")]
    IoError(#[from] std::io::Error),

    #[error(transparent)]
    VsockProtocolError(#[from] VsockProtocolHandlerError),
}

#[derive(Error, Debug)]
pub enum VsockProtocolHandlerError {
    #[error("Invalid chunk data length")]
    InvalidPayloadLengthError,
    #[error("Invalid total data length")]
    InvalidTotalLengthError,
    #[error("Invalid chunk index")]
    InvalidChunkIndexError,
    #[error("message too large error")]
    MessageTooLargeError,
    #[error("IO error")]
    IoError(#[from] std::io::Error),
}
