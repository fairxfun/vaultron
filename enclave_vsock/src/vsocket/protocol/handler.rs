use super::{MessageHeader, ProtocolStreamTrait, HEADER_SIZE, MAX_VSOCK_MESSAGE_SIZE, MAX_VSOCK_TOTAL_PAYLOAD_SIZE};
use crate::error::VsockProtocolError;
use typed_builder::TypedBuilder;

#[derive(Debug, Clone, TypedBuilder)]
pub struct VsockProtocol {}

impl VsockProtocol {
    /// Read message (supports large messages)
    pub fn read_message<T: ProtocolStreamTrait>(&self, stream: &mut T) -> Result<Vec<u8>, VsockProtocolError> {
        let mut header_bytes = [0u8; HEADER_SIZE];
        stream.read_exact(&mut header_bytes)?;

        let header = MessageHeader::from_bytes(&header_bytes)?;
        if header.total_data_length as usize > MAX_VSOCK_TOTAL_PAYLOAD_SIZE {
            return Err(VsockProtocolError::MessageTooLargeError);
        }

        let mut payload = vec![0u8; header.total_data_length as usize];
        let mut bytes_read = 0;

        while bytes_read < header.total_data_length as usize {
            let remaining = header.total_data_length as usize - bytes_read;
            let chunk_size = std::cmp::min(remaining, MAX_VSOCK_MESSAGE_SIZE);
            stream.read_exact(&mut payload[bytes_read..bytes_read + chunk_size])?;
            bytes_read += chunk_size;
        }

        Ok(payload)
    }

    /// Write message (automatic chunking for large messages)
    pub fn write_message<T: ProtocolStreamTrait>(
        &self,
        stream: &mut T,
        message: &[u8],
    ) -> Result<(), VsockProtocolError> {
        let total_message_len = message.len();
        let header = MessageHeader::builder()
            .total_data_length(total_message_len as u32)
            .build();
        let header_vec = header.to_vec()?;
        stream.write_all(&header_vec)?;

        let mut bytes_written = 0;
        while bytes_written < total_message_len {
            let remaining = total_message_len - bytes_written;
            let chunk_size = std::cmp::min(remaining, MAX_VSOCK_MESSAGE_SIZE);
            stream.write_all(&message[bytes_written..bytes_written + chunk_size])?;
            bytes_written += chunk_size;
        }

        stream.flush()?;
        Ok(())
    }
}

pub fn create_vsock_protocol() -> VsockProtocol {
    VsockProtocol::builder().build()
}
