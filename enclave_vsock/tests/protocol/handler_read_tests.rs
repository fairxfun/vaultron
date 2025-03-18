use crate::protocol::{
    protocol_mock::MockProtocolStream,
    utils::{random_bytes, random_len},
};
use enclave_vsock::{
    create_vsock_protocol, MessageHeader, HEADER_SIZE, MAX_VSOCK_MESSAGE_SIZE, MAX_VSOCK_TOTAL_PAYLOAD_SIZE,
};
use mockall::predicate;

#[tokio::test]
pub async fn test_vsock_protocol_read_single_chunk_message() {
    let protocol = create_vsock_protocol();
    let mut mock_stream = MockProtocolStream::new();

    let message_len = random_len(1, MAX_VSOCK_MESSAGE_SIZE);
    let message_lens = [message_len, MAX_VSOCK_MESSAGE_SIZE];
    for message_len in message_lens {
        let message = random_bytes(message_len);
        let header = MessageHeader::builder().total_data_length(message_len as u32).build();
        let header_vec = header.to_vec().unwrap();

        // Setup the mock to handle the header read
        mock_stream
            .expect_read_exact()
            .times(1)
            .with(predicate::always())
            .returning(move |buf| {
                if buf.len() == HEADER_SIZE {
                    buf.copy_from_slice(&header_vec);
                }
                Ok(())
            });

        // Clone the message for the second mock
        let message_clone = message.clone();

        // Setup the mock to handle the message read
        mock_stream
            .expect_read_exact()
            .times(1)
            .with(predicate::always())
            .returning(move |buf| {
                if buf.len() == message_len {
                    buf.copy_from_slice(&message_clone);
                }
                Ok(())
            });

        let result = protocol.read_message(&mut mock_stream).unwrap();
        assert_eq!(result, message);
    }
}

#[tokio::test]
pub async fn test_vsock_protocol_read_multiple_chunk_message() {
    let protocol = create_vsock_protocol();
    let mut mock_stream = MockProtocolStream::new();

    let message_len = random_len(MAX_VSOCK_MESSAGE_SIZE + 1, MAX_VSOCK_TOTAL_PAYLOAD_SIZE);
    let message_lens = [message_len, MAX_VSOCK_TOTAL_PAYLOAD_SIZE];
    for message_len in message_lens {
        let message = random_bytes(message_len);
        let message_clone = message.clone();
        let header = MessageHeader::builder().total_data_length(message_len as u32).build();
        let header_vec = header.to_vec().unwrap();

        mock_stream
            .expect_read_exact()
            .times(1)
            .with(predicate::always())
            .returning(move |buf| {
                if buf.len() == HEADER_SIZE {
                    buf.copy_from_slice(&header_vec);
                }
                Ok(())
            });

        let full_chunks = message_len / MAX_VSOCK_MESSAGE_SIZE;
        let last_chunk_size = message_len % MAX_VSOCK_MESSAGE_SIZE;

        // Handle full chunks
        for i in 0..full_chunks {
            let start = i * MAX_VSOCK_MESSAGE_SIZE;
            let end = start + MAX_VSOCK_MESSAGE_SIZE;
            let message_chunk = message[start..end].to_vec();
            let message_chunk_clone = message_chunk.clone();

            mock_stream
                .expect_read_exact()
                .times(1)
                .with(predicate::always())
                .returning(move |buf| {
                    if buf.len() == MAX_VSOCK_MESSAGE_SIZE {
                        buf.copy_from_slice(&message_chunk_clone);
                    }
                    Ok(())
                });
        }

        // Handle the last partial chunk if it exists
        if last_chunk_size > 0 {
            let start = full_chunks * MAX_VSOCK_MESSAGE_SIZE;
            let message_chunk = message[start..].to_vec();
            let message_chunk_clone = message_chunk.clone();

            mock_stream
                .expect_read_exact()
                .times(1)
                .with(predicate::always())
                .returning(move |buf| {
                    if buf.len() == last_chunk_size {
                        buf.copy_from_slice(&message_chunk_clone);
                    }
                    Ok(())
                });
        }

        let result = protocol.read_message(&mut mock_stream).unwrap();
        assert_eq!(result, message_clone);
    }
}
