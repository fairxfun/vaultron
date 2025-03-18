use crate::protocol::{
    protocol_mock::MockProtocolStream,
    utils::{random_bytes, random_len},
};
use enclave_vsock::{
    create_vsock_protocol, MessageHeader, VsockProtocolError, HEADER_SIZE, MAX_VSOCK_MESSAGE_SIZE,
    MAX_VSOCK_TOTAL_PAYLOAD_SIZE,
};
use mockall::predicate;

#[tokio::test]
pub async fn test_vsock_protocol_write_single_chunk_message() {
    let protocol = create_vsock_protocol();
    let mut mock_stream = MockProtocolStream::new();

    let message_len = random_len(1, MAX_VSOCK_MESSAGE_SIZE);
    let message_lens = [message_len, MAX_VSOCK_MESSAGE_SIZE];
    for message_len in message_lens {
        let message = random_bytes(message_len);
        let message_clone = message.clone();
        let header = MessageHeader::builder().total_data_length(message_len as u32).build();
        let header_vec = header.to_vec().unwrap();

        mock_stream
            .expect_write_all()
            .times(1)
            .with(predicate::always())
            .returning(move |buf| {
                if buf.len() == HEADER_SIZE && buf == header_vec {
                    Ok(())
                } else {
                    Err(VsockProtocolError::InvalidMessageDataError)
                }
            });

        mock_stream
            .expect_write_all()
            .times(1)
            .with(predicate::always())
            .returning(move |buf| {
                if buf.len() == message_len && buf == message {
                    Ok(())
                } else {
                    Err(VsockProtocolError::InvalidMessageDataError)
                }
            });

        mock_stream.expect_flush().times(1).returning(move || Ok(()));

        let result = protocol.write_message(&mut mock_stream, &message_clone);
        assert!(result.is_ok());
    }
}

#[tokio::test]
pub async fn test_vsock_protocol_write_multiple_chunk_message() {
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
            .expect_write_all()
            .times(1)
            .with(predicate::always())
            .returning(move |buf| {
                if buf.len() == HEADER_SIZE && buf == header_vec {
                    Ok(())
                } else {
                    Err(VsockProtocolError::InvalidMessageDataError)
                }
            });

        let full_chunks = message_len / MAX_VSOCK_MESSAGE_SIZE;
        let last_chunk_size = message_len % MAX_VSOCK_MESSAGE_SIZE;

        for i in 0..full_chunks {
            let start = i * MAX_VSOCK_MESSAGE_SIZE;
            let end = start + MAX_VSOCK_MESSAGE_SIZE;
            let message_chunk = message[start..end].to_vec();
            let message_chunk_clone = message_chunk.clone();

            mock_stream
                .expect_write_all()
                .times(1)
                .with(predicate::always())
                .returning(move |buf| {
                    if buf.len() == MAX_VSOCK_MESSAGE_SIZE && buf == message_chunk_clone {
                        Ok(())
                    } else {
                        Err(VsockProtocolError::InvalidMessageDataError)
                    }
                });
        }

        if last_chunk_size > 0 {
            let start = full_chunks * MAX_VSOCK_MESSAGE_SIZE;
            let message_chunk = message[start..].to_vec();
            let message_chunk_clone = message_chunk.clone();

            mock_stream
                .expect_write_all()
                .times(1)
                .with(predicate::always())
                .returning(move |buf| {
                    if buf.len() == last_chunk_size && buf == message_chunk_clone {
                        Ok(())
                    } else {
                        Err(VsockProtocolError::InvalidMessageDataError)
                    }
                });
        }

        mock_stream.expect_flush().times(1).returning(move || Ok(()));

        let result = protocol.write_message(&mut mock_stream, &message_clone);
        assert!(result.is_ok());
    }
}
