use thiserror::Error;

#[derive(Error, Debug)]
pub enum VsockServerError {
    #[error("IO error")]
    IoError(#[from] std::io::Error),
}

#[derive(Error, Debug)]
pub enum VsockClientError {
    #[error("Stream lock error")]
    StreamLockError,

    #[error("IO error")]
    IoError(#[from] std::io::Error),

    #[error(transparent)]
    VsockProtocolError(#[from] VsockProtocolError),
}

#[derive(Error, Debug)]
pub enum VsockProtocolError {
    #[error("Invalid magic bytes error")]
    InvalidMagicBytesError,

    #[error("Invalid chunk data length error")]
    InvalidPayloadLengthError,

    #[error("Invalid total data length error")]
    InvalidTotalLengthError,

    #[error("Invalid chunk index error")]
    InvalidChunkIndexError,

    #[error("Message too large error")]
    MessageTooLargeError,

    #[error("Invalid message data error")]
    InvalidMessageDataError,

    #[error("IO error")]
    IoError(#[from] std::io::Error),
}
