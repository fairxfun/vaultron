use crate::error::EnclaveSocketError;
use log::{debug, error};
use std::{
    f32::consts::{E, LOG10_2},
    fmt,
    io::{Read, Write},
};
use typed_builder::TypedBuilder;
use vsock::VsockStream;

use super::{ChunkHeader, CHUNK_HEADER_SIZE, MAX_VSOCK_MESSAGE_SIZE, MAX_VSOCK_TOTAL_MESSAGE_SIZE};

/// A generic trait for message protocol implementations
/// Allows customizing the error type through the associated type
pub trait MessageProtocol {
    /// The error type that will be returned by protocol operations
    type Error;

    /// Read a message from the provided stream
    fn read_message<R: Read>(&self, stream: &mut R) -> Result<Vec<u8>, Self::Error>;

    /// Write a message to the provided stream
    fn write_message<W: Write>(&self, stream: &mut W, message: &[u8]) -> Result<(), Self::Error>;
}

#[derive(Debug, Clone, TypedBuilder)]
pub struct VsockMessageProtocol {}

impl MessageProtocol for VsockMessageProtocol {
    type Error = EnclaveSocketError;

    fn read_message<R: Read>(&self, stream: &mut R) -> Result<Vec<u8>, Self::Error> {
        // Read header (8 bytes)
        let mut header_buf = [0u8; CHUNK_HEADER_SIZE];
        stream.read_exact(&mut header_buf)?;

        // Parse header
        let header = ChunkHeader::from_vec(&header_buf);
        header.validate()?;

        if header.is_single_chunk() {
            // Read payload
            let mut payload = vec![0u8; header.chunk_data_length as usize];
            stream.read_exact(&mut payload)?;

            return Ok(payload);
        } else {
            // Multi-chunk message, initialize reassembly buffer
            let mut buffer = vec![0u8; header.total_data_length as usize];

            let mut payload = vec![0u8; header.chunk_data_length as usize];
            stream.read_exact(&mut payload)?;

            buffer[0..payload.len()].copy_from_slice(&payload);

            loop {
                let mut header_buf = [0u8; CHUNK_HEADER_SIZE];
                stream.read_exact(&mut header_buf)?;

                let header = ChunkHeader::from_vec(&header_buf);
                header.validate()?;

                let mut payload = vec![0u8; header.chunk_data_length as usize];
                stream.read_exact(&mut payload)?;

                buffer[header.chunk_index as usize * MAX_VSOCK_MESSAGE_SIZE
                    ..header.chunk_index as usize * MAX_VSOCK_MESSAGE_SIZE + payload.len()]
                    .copy_from_slice(&payload);

                if header.is_last_chunk() {
                    if buffer.len() != header.total_data_length as usize {
                        return Err(EnclaveSocketError::InvalidTotalLengthError);
                    }

                    return Ok(buffer);
                }
            }
        }
    }

    fn write_message<W: Write>(&self, stream: &mut W, message: &[u8]) -> Result<(), Self::Error> {
        if message.len() > MAX_VSOCK_TOTAL_MESSAGE_SIZE {
            return Err(EnclaveSocketError::MessageTooLargeError);
        }

        let max_payload = MAX_VSOCK_MESSAGE_SIZE - CHUNK_HEADER_SIZE;

        if message.len() <= max_payload {
            // Single chunk message
            let header = ChunkHeader::builder()
                .total_data_length(message.len() as u32) // Total Data Length - size of payload only
                .chunk_index(0u16)
                .chunk_data_length(message.len() as u16) // Chunk Data Length - size of this chunk's payload
                .build();

            // Write header
            stream.write_all(&header.to_vec())?;

            // Write payload
            stream.write_all(message)?;
        } else {
            // Multi-chunk message
            let total_len = message.len() as u32; // Total Data Length - size of all payload data combined
            let num_chunks = (message.len() + max_payload - 1) / max_payload;

            debug!("Sending {} bytes in {} chunks", message.len(), num_chunks);

            for chunk_idx in 0..num_chunks {
                let start = chunk_idx * max_payload;
                let end = std::cmp::min(start + max_payload, message.len());
                let payload = &message[start..end];
                let chunk_len = payload.len() as u16; // Chunk Data Length - size of this chunk's payload

                let header = ChunkHeader::builder()
                    .total_data_length(total_len) // Total Data Length - size of all payload data combined
                    .chunk_index(chunk_idx as u16)
                    .chunk_data_length(chunk_len) // Chunk Data Length - size of this chunk's payload
                    .build();

                // Write header
                stream.write_all(&header.to_vec())?;
                // Write payload
                stream.write_all(payload)?;
            }
        }

        stream.flush()?;
        Ok(())
    }
}

// For backward compatibility, keep the original methods
impl VsockMessageProtocol {
    /// Read message (supports chunk reassembly)
    pub fn read_message(&self, stream: &mut VsockStream) -> Result<Vec<u8>, EnclaveSocketError> {
        <Self as MessageProtocol>::read_message(self, stream)
    }

    /// Write message (automatic chunking)
    pub fn write_message(&self, stream: &mut VsockStream, message: &[u8]) -> Result<(), EnclaveSocketError> {
        <Self as MessageProtocol>::write_message(self, stream, message)
    }
}

pub fn create_vsock_message_protocol() -> VsockMessageProtocol {
    VsockMessageProtocol::builder().build()
}

// Example of a custom implementation with a different error type
// ============================================================

/// Custom error type for the SimpleMessageProtocol
#[derive(Debug)]
pub enum SimpleProtocolError {
    MessageTooLarge(usize),
    IoError(std::io::Error),
    InvalidHeader,
}

impl fmt::Display for SimpleProtocolError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SimpleProtocolError::MessageTooLarge(size) => write!(f, "Message too large: {} bytes", size),
            SimpleProtocolError::IoError(err) => write!(f, "IO error: {}", err),
            SimpleProtocolError::InvalidHeader => write!(f, "Invalid message header"),
        }
    }
}

impl std::error::Error for SimpleProtocolError {}

impl From<std::io::Error> for SimpleProtocolError {
    fn from(err: std::io::Error) -> Self {
        SimpleProtocolError::IoError(err)
    }
}

/// A simpler message protocol implementation with a custom error type
#[derive(Debug, Clone)]
pub struct SimpleMessageProtocol {
    max_message_size: usize,
}

impl SimpleMessageProtocol {
    pub fn new(max_message_size: usize) -> Self {
        Self { max_message_size }
    }
}

impl MessageProtocol for SimpleMessageProtocol {
    type Error = SimpleProtocolError;

    fn read_message<R: Read>(&self, stream: &mut R) -> Result<Vec<u8>, Self::Error> {
        // Read message length (4 bytes)
        let mut len_buf = [0u8; 4];
        stream.read_exact(&mut len_buf)?;

        let message_len = u32::from_be_bytes(len_buf) as usize;

        if message_len > self.max_message_size {
            return Err(SimpleProtocolError::MessageTooLarge(message_len));
        }

        // Read message data
        let mut message = vec![0u8; message_len];
        stream.read_exact(&mut message)?;

        Ok(message)
    }

    fn write_message<W: Write>(&self, stream: &mut W, message: &[u8]) -> Result<(), Self::Error> {
        if message.len() > self.max_message_size {
            return Err(SimpleProtocolError::MessageTooLarge(message.len()));
        }

        // Write message length
        let len_bytes = (message.len() as u32).to_be_bytes();
        stream.write_all(&len_bytes)?;

        // Write message data
        stream.write_all(message)?;
        stream.flush()?;

        Ok(())
    }
}
