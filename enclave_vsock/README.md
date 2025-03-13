# Enclave Socket

This module provides socket communication functionality for the enclave system, with a focus on VSOCK communication that supports message chunking for large payloads.

## VSOCK Message Chunking Protocol

VSOCK is a stream-based protocol (like TCP) that doesn't preserve message boundaries. The VSOCK Message Chunking Protocol allows for sending messages larger than the maximum VSOCK message size (64KB) by splitting them into multiple chunks and reassembling them on the receiving end.

### Message Format

```
+------------------+----------------+----------------+------------------+
| Total Data Length| Chunk Index    | Chunk Data Length| Payload        |
| (4 bytes)        | (2 bytes)      | (2 bytes)      | (variable)       |
+------------------+----------------+----------------+------------------+
```

#### Total Data Length: 4 bytes (u32, big-endian)
- Total length of the complete message payload after reassembly
- This is the size of the final payload after all chunks are combined
- Does not include the header length

#### Chunk Index: 2 bytes (u16, big-endian)
- 0-based index of this chunk in the sequence
- Indicates the position of this chunk in the complete message

#### Chunk Data Length: 2 bytes (u16, big-endian)
- Length of the payload data in this specific chunk
- Allows for efficient buffer allocation when processing chunks
- Does not include the header length

#### Chunk Header is 8 bytes:
- Total Data Length: 4 bytes (u32, big-endian)
- Chunk Index: 2 bytes (u16, big-endian)
- Chunk Data Length: 2 bytes (u16, big-endian)

#### Payload: Variable length
- Actual message data
- Maximum size = MAX_VSOCK_MESSAGE_SIZE (65535) - CHUNK_HEADER_SIZE (8)

### Chunking Process

For messages larger than MAX_VSOCK_MESSAGE_SIZE - CHUNK_HEADER_SIZE:

1. Split into multiple chunks
2. First chunk has index 0
3. Chunks are sent in sequence
4. Receiver reassembles based on chunk index and total data length

### Communication Flow

1. Sender writes 8-byte header (Total Data Length + Chunk Index + Chunk Data Length)
2. Sender writes payload data
3. Receiver reads 8-byte header
4. Receiver reads payload based on Chunk Data Length
5. Receiver processes the chunk (either returns it directly or reassembles)

### Implementation Details

- Maximum chunk size: 65535 bytes (64KB)
- Maximum total message size: 1MB (16 chunks)
- Chunking is handled automatically by the `EnclaveVSocket` class
- The implementation is stateless - reassembly is handled within the read_message function
- The `ChunkHeader` struct provides convenient methods for header serialization and deserialization

## Message Protocol Trait

The module provides a generic `MessageProtocol` trait that allows for different message protocol implementations with customizable error types:

```rust
pub trait MessageProtocol {
    /// The error type that will be returned by protocol operations
    type Error;

    /// Read a message from the provided stream
    fn read_message<R: Read>(&self, stream: &mut R) -> Result<Vec<u8>, Self::Error>;

    /// Write a message to the provided stream
    fn write_message<W: Write>(&self, stream: &mut W, message: &[u8]) -> Result<(), Self::Error>;
}
```

### Default Implementation

The `VsockMessageProtocol` struct implements the `MessageProtocol` trait with `EnclaveSocketError` as its error type:

```rust
impl MessageProtocol for VsockMessageProtocol {
    type Error = EnclaveSocketError;
    
    // Implementation of read_message and write_message...
}
```

### Custom Implementations

You can create your own message protocol implementations with custom error types:

```rust
// Define a custom error type
#[derive(Debug)]
pub enum MyCustomError {
    MessageTooLarge(usize),
    IoError(std::io::Error),
    // Other error variants...
}

// Implement the MessageProtocol trait with your custom error type
impl MessageProtocol for MyCustomProtocol {
    type Error = MyCustomError;
    
    fn read_message<R: Read>(&self, stream: &mut R) -> Result<Vec<u8>, Self::Error> {
        // Your implementation...
    }
    
    fn write_message<W: Write>(&self, stream: &mut W, message: &[u8]) -> Result<(), Self::Error> {
        // Your implementation...
    }
}
```

## Usage

The `EnclaveVSocket` class provides methods for reading and writing messages with automatic chunking support:

```rust
// Create a new socket handler
let mut socket = create_enclave_vsocket();

// Write a message (automatically chunks if necessary)
socket.write_message(&mut stream, &large_message)?;

// Read a message (automatically reassembles chunks)
let complete_message = socket.read_message(&mut stream)?;
```

Using a custom protocol implementation:

```rust
// Create a custom protocol handler
let custom_protocol = SimpleMessageProtocol::new(1024 * 1024); // 1MB max message size

// Write a message using the custom protocol
custom_protocol.write_message(&mut stream, &message)?;

// Read a message using the custom protocol
let received_message = custom_protocol.read_message(&mut stream)?;
```

## Error Handling

The protocol includes validation for:
- Chunk data length limits (`InvalidPayloadLengthError`)
- Total message size limits (`MessageTooLargeError`)
- Chunk sequence integrity (`InvalidChunkIndexError`)
- Total data length consistency (`InvalidTotalLengthError`)
- Data overflow protection
- Reassembly state consistency 

Custom implementations can define their own error types and handling strategies. 