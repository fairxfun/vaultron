/*
    VSOCK Message Chunking Protocol
    ==============================

    VSOCK is a stream-based protocol (like TCP) that doesn't preserve message boundaries.
    This protocol defines a simple chunking mechanism for large messages.

    Message Format:
    +------------------+----------------+----------------+------------------+
    | Total Data Length| Chunk Index    | Chunk Data Length| Payload        |
    | (4 bytes)        | (2 bytes)      | (2 bytes)      | (variable)       |
    +------------------+----------------+----------------+------------------+

    Total Data Length: 4 bytes (u32, big-endian)
      - Total length of the complete message payload after reassembly
      - This is the size of the final payload after all chunks are combined
      - Does not include the header length

    Chunk Index: 2 bytes (u16, big-endian)
      - 0-based index of this chunk in the sequence
      - Indicates the position of this chunk in the complete message

    Chunk Data Length: 2 bytes (u16, big-endian)
      - Length of the payload data in this specific chunk
      - Allows for efficient buffer allocation when processing chunks
      - Does not include the header length

    Chunk Header is 8 bytes:
      - Total Data Length: 4 bytes (u32, big-endian)
      - Chunk Index: 2 bytes (u16, big-endian)
      - Chunk Data Length: 2 bytes (u16, big-endian)

    Payload: Variable length
      - Actual message data
      - Maximum size = MAX_VSOCK_MESSAGE_SIZE (65535) - CHUNK_HEADER_SIZE (8)

    For messages larger than MAX_VSOCK_MESSAGE_SIZE - CHUNK_HEADER_SIZE:
    1. Split into multiple chunks
    2. First chunk has index 0
    3. Chunks are sent in sequence
    4. Receiver reassembles based on chunk index and total data length

    Communication Flow:
    1. Sender writes 8-byte header (Total Data Length + Chunk Index + Chunk Data Length)
    2. Sender writes payload data
    3. Receiver reads 8-byte header
    4. Receiver reads payload based on Chunk Data Length
    5. Receiver processes the chunk (either returns it directly or reassembles)
*/

use typed_builder::TypedBuilder;

use crate::VsockProtocolHandlerError;

pub const MAX_VSOCK_MESSAGE_SIZE: usize = 65535; // 64K
pub const MAX_VSOCK_TOTAL_MESSAGE_SIZE: usize = MAX_VSOCK_MESSAGE_SIZE * 16; // 1M

// Total Data Length + Chunk Index + Chunk Data Length
pub const CHUNK_HEADER_SIZE: usize = 8; // 4 + 2 + 2 bytes
pub const MAX_VSOCK_PAYLOAD_SIZE: usize = MAX_VSOCK_MESSAGE_SIZE - CHUNK_HEADER_SIZE; // 65527

#[derive(Debug, Clone, TypedBuilder)]
pub struct ChunkHeader {
    pub total_data_length: u32,
    pub chunk_index: u16,
    pub chunk_data_length: u16,
}

impl ChunkHeader {
    pub fn to_vec(&self) -> Vec<u8> {
        let mut vec = Vec::new();
        vec.extend_from_slice(&self.total_data_length.to_be_bytes());
        vec.extend_from_slice(&self.chunk_index.to_be_bytes());
        vec.extend_from_slice(&self.chunk_data_length.to_be_bytes());
        vec
    }

    pub fn from_vec(vec: &[u8; CHUNK_HEADER_SIZE]) -> Self {
        let total_data_length = u32::from_be_bytes([vec[0], vec[1], vec[2], vec[3]]);
        let chunk_index = u16::from_be_bytes([vec[4], vec[5]]);
        let chunk_data_length = u16::from_be_bytes([vec[6], vec[7]]);
        Self::builder()
            .total_data_length(total_data_length)
            .chunk_index(chunk_index)
            .chunk_data_length(chunk_data_length)
            .build()
    }

    pub fn validate(&self) -> Result<(), VsockProtocolHandlerError> {
        if self.total_data_length as usize > MAX_VSOCK_TOTAL_MESSAGE_SIZE {
            return Err(VsockProtocolHandlerError::InvalidTotalLengthError);
        }

        if self.chunk_data_length as usize > MAX_VSOCK_MESSAGE_SIZE - CHUNK_HEADER_SIZE {
            return Err(VsockProtocolHandlerError::InvalidPayloadLengthError);
        }

        Ok(())
    }

    pub fn is_single_chunk(&self) -> bool {
        self.chunk_index == 0 && self.total_data_length as usize == self.chunk_data_length as usize
    }

    pub fn is_last_chunk(&self) -> bool {
        // Calculate total number of chunks needed (ceiling division)
        let total_chunks = (self.total_data_length as usize).div_ceil(MAX_VSOCK_PAYLOAD_SIZE);

        // Check if current chunk is the last one (0-based indexing)
        self.chunk_index as usize == total_chunks - 1
    }
}
