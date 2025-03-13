use anyhow::Result;
use std::io::{Read, Write};
use thiserror::Error;
use vsock::VsockStream;

#[derive(Debug, Error)]
pub enum EnclaveSocketError {
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Invalid message format: {0}")]
    InvalidMessageFormat(String),

    #[error("Message too large: {0}")]
    MessageTooLarge(usize),
}

/// Read a message from a VSOCK stream
pub fn read_message(stream: &mut VsockStream) -> Result<Vec<u8>, EnclaveSocketError> {
    // Basic implementation - will be expanded later
    let mut length_buf = [0u8; 4];
    stream.read_exact(&mut length_buf)?;

    let length = u32::from_be_bytes(length_buf) as usize;
    let mut buffer = vec![0u8; length];
    stream.read_exact(&mut buffer)?;

    Ok(buffer)
}

/// Write a message to a VSOCK stream
pub fn write_message(stream: &mut VsockStream, message: &[u8]) -> Result<(), EnclaveSocketError> {
    // Basic implementation - will be expanded later
    let length = message.len() as u32;
    let length_bytes = length.to_be_bytes();

    stream.write_all(&length_bytes)?;
    stream.write_all(message)?;

    Ok(())
}
