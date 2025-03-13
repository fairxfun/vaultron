use crate::error::VsockProtocolHandlerError;
use log::debug;
use std::io::{Read, Write};
use typed_builder::TypedBuilder;
use vsock::VsockStream;

use super::{ChunkHeader, CHUNK_HEADER_SIZE, MAX_VSOCK_MESSAGE_SIZE, MAX_VSOCK_TOTAL_MESSAGE_SIZE};

#[derive(Debug, Clone, TypedBuilder)]
pub struct VsockProtocol {}

impl VsockProtocol {
    /// Read message (supports chunk reassembly)
    pub fn read_message(&self, stream: &mut VsockStream) -> Result<Vec<u8>, VsockProtocolHandlerError> {
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
            Ok(payload)
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
                        return Err(VsockProtocolHandlerError::InvalidTotalLengthError);
                    }

                    return Ok(buffer);
                }
            }
        }
    }

    /// Write message (automatic chunking)
    pub fn write_message(&self, stream: &mut VsockStream, message: &[u8]) -> Result<(), VsockProtocolHandlerError> {
        if message.len() > MAX_VSOCK_TOTAL_MESSAGE_SIZE {
            return Err(VsockProtocolHandlerError::MessageTooLargeError);
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
            let num_chunks = message.len().div_ceil(max_payload);

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

pub fn create_vsock_protocol() -> VsockProtocol {
    VsockProtocol::builder().build()
}
