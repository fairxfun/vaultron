# Enclave Socket

This module provides socket communication functionality for the enclave system, with a focus on VSOCK communication that supports message chunking for large payloads.

## VSOCK Message Protocol

VSOCK is a stream-based protocol (like TCP) that doesn't preserve message boundaries. The VSOCK Message Protocol allows for sending messages larger than the maximum VSOCK message size (64KB) by using a simple header and handling large messages with multiple writes.

### Message Format

```
+------------------+------------------+------------------+
| Magic Bytes      | Total Data Length| Payload          |
| (4 bytes)        | (4 bytes)        | (variable)       |
+------------------+------------------+------------------+
```

#### Magic Bytes: 4 bytes
- Fixed value "VSOK" (0x56, 0x53, 0x4F, 0x4B in hex)
- Used to identify the start of a valid message header
- Helps with protocol validation and error detection

#### Total Data Length: 4 bytes (u32, big-endian)
- Total length of the message payload
- Does not include the header length
- **Must be greater than 0** (empty messages are not allowed)

#### Message Header is 8 bytes:
- Magic Bytes: 4 bytes (fixed value "VSOK")
- Total Data Length: 4 bytes (u32, big-endian)

#### Payload: Variable length
- Actual message data
- Maximum size = 1MB (defined by MAX_VSOCK_TOTAL_PAYLOAD_SIZE)

### Important Protocol Requirements

1. **Magic Bytes Validation**: All valid messages must begin with the magic bytes "VSOK". This helps identify the start of a message and validate that the data conforms to the protocol.

2. **No Empty Messages**: All messages must contain at least one byte of data. The protocol does not support empty messages, and the Total Data Length must always be greater than 0.

### Large Message Handling

For messages larger than MAX_VSOCK_MESSAGE_SIZE:

1. The header is written once with the total message size
2. The payload is written in multiple consecutive writes
3. The receiver reads the entire payload based on the Total Data Length

### Communication Flow

1. Sender writes 8-byte header (Magic Bytes + Total Data Length)
2. Sender writes payload data (in multiple writes if necessary)
3. Receiver reads 8-byte header
4. Receiver validates the magic bytes
5. Receiver reads payload based on Total Data Length (in multiple reads if necessary)

### Implementation Details

- Maximum message size: 1MB (defined by MAX_VSOCK_TOTAL_PAYLOAD_SIZE)
- Header size: 8 bytes
- Chunking is handled automatically by the `VsockProtocol` class
- The implementation is stateless - message handling is done within the read_message and write_message functions
- The `MessageHeader` struct provides convenient methods for header serialization and deserialization

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

The `VsockProtocol` struct implements the `MessageProtocol` trait with `VsockProtocolError` as its error type:

```rust
impl MessageProtocol for VsockProtocol {
    type Error = VsockProtocolError;
    
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

The `EnclaveVSocket` class provides methods for reading and writing messages:

```rust
// Create a new socket handler
let mut socket = create_enclave_vsocket();

// Write a message (handles large messages automatically)
socket.write_message(&mut stream, &large_message)?;

// Read a message
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
- Magic bytes validation (`InvalidMagicBytesError`)
- Total message size limits (`MessageTooLargeError`)
- Total data length validation (`InvalidTotalLengthError`)
- I/O errors (`IoError`)

Custom implementations can define their own error types and handling strategies. 