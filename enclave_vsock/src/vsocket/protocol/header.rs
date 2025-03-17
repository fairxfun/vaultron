/*
    VSOCK Message Chunking Protocol
    ==============================

    VSOCK is a stream-based protocol (like TCP) that doesn't preserve message boundaries.
    This protocol defines a simple chunking mechanism for large messages.

    Message Header Format:
    +------------------+------------------+
    | Magic Bytes      | Total Data Length|
    | (4 bytes)        | (4 bytes)        |
    +------------------+------------------+

    Magic Bytes: 4 bytes
    - Fixed value "VSOK" (0x56, 0x53, 0x4F, 0x4B in hex)
    - Used to identify the start of a valid message header
    - Helps with protocol validation and error detection

    Total Data Length: 4 bytes (u32, big-endian)
    - Total length of the complete message payload
    - Does not include the header length
    - Must be greater than 0

    Single Message:
    - Message Header(8 bytes) + Message data(data size)

    Multiple Chunks:
    - If message size > MAX_VSOCK_MESSAGE_SIZE, data is sent in multiple consecutive writes
    - All chunks share the same header
    - Receiver reads the entire message based on Total Data Length
*/

use typed_builder::TypedBuilder;

use crate::VsockProtocolError;

// Magic bytes constant - "VSOK" in ASCII
pub const VSOCK_MAGIC_BYTES: [u8; 4] = [0x56, 0x53, 0x4F, 0x4B];

// Header size constants
pub const HEADER_SIZE: usize = 8; // 4(magic) + 4(total length)
pub const MAX_VSOCK_MESSAGE_SIZE: usize = 65536; // 64K
pub const MAX_VSOCK_CHUNK_COUNT: usize = 16;
pub const MAX_VSOCK_TOTAL_PAYLOAD_SIZE: usize = MAX_VSOCK_MESSAGE_SIZE * MAX_VSOCK_CHUNK_COUNT;

#[derive(Debug, Clone, TypedBuilder, PartialEq, Eq)]
pub struct MessageHeader {
    pub total_data_length: u32,
}

impl MessageHeader {
    pub fn to_vec(&self) -> Result<Vec<u8>, VsockProtocolError> {
        if self.total_data_length as usize > MAX_VSOCK_TOTAL_PAYLOAD_SIZE {
            return Err(VsockProtocolError::MessageTooLargeError);
        }

        if self.total_data_length == 0 {
            return Err(VsockProtocolError::InvalidTotalLengthError);
        }

        let mut buf = Vec::with_capacity(HEADER_SIZE);
        buf.extend_from_slice(&VSOCK_MAGIC_BYTES);
        buf.extend_from_slice(&self.total_data_length.to_be_bytes());
        Ok(buf)
    }

    pub fn from_bytes(bytes: &[u8; HEADER_SIZE]) -> Result<Self, VsockProtocolError> {
        if bytes[0..4] != VSOCK_MAGIC_BYTES {
            return Err(VsockProtocolError::InvalidMagicBytesError);
        }

        let total_data_length = u32::from_be_bytes([bytes[4], bytes[5], bytes[6], bytes[7]]);

        if total_data_length == 0 {
            return Err(VsockProtocolError::InvalidTotalLengthError);
        }

        if total_data_length as usize > MAX_VSOCK_TOTAL_PAYLOAD_SIZE {
            return Err(VsockProtocolError::MessageTooLargeError);
        }
        Ok(Self { total_data_length })
    }
}
