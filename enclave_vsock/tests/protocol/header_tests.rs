use enclave_vsock::{MessageHeader, HEADER_SIZE, MAX_VSOCK_TOTAL_PAYLOAD_SIZE, VSOCK_MAGIC_BYTES};

#[tokio::test]
pub async fn test_chunk_header_to_vec() {
    let valid_size = MAX_VSOCK_TOTAL_PAYLOAD_SIZE / 2;
    let header = MessageHeader::builder().total_data_length(valid_size as u32).build();

    let header_vec = header.to_vec().unwrap();
    assert_eq!(header_vec.len(), HEADER_SIZE);
    assert_eq!(header_vec[0..4], VSOCK_MAGIC_BYTES);
    assert_eq!(header_vec[4..8], (valid_size as u32).to_be_bytes());

    let header_bytes: [u8; HEADER_SIZE] = header_vec.try_into().unwrap();
    let header_from_bytes = MessageHeader::from_bytes(&header_bytes).unwrap();
    assert_eq!(header, header_from_bytes);

    let invalid_size = MAX_VSOCK_TOTAL_PAYLOAD_SIZE + 1;
    let header = MessageHeader::builder().total_data_length(invalid_size as u32).build();
    let header_vec = header.to_vec();
    assert!(header_vec.is_err());

    let invalid_size = 0;
    let header = MessageHeader::builder().total_data_length(invalid_size as u32).build();
    let header_vec = header.to_vec();
    assert!(header_vec.is_err());
}

#[tokio::test]
pub async fn test_chunk_header_from_bytes() {
    let mut header_bytes = [0u8; HEADER_SIZE];
    header_bytes[0..4].copy_from_slice(&[0x00, 0x00, 0x00, 0x00]);
    header_bytes[4..8].copy_from_slice(&(1024u32).to_be_bytes());
    let result = MessageHeader::from_bytes(&header_bytes);
    assert!(result.is_err());

    header_bytes[0..4].copy_from_slice(&VSOCK_MAGIC_BYTES);
    header_bytes[4..8].copy_from_slice(&(0u32).to_be_bytes());
    let result = MessageHeader::from_bytes(&header_bytes);
    assert!(result.is_err());

    header_bytes[0..4].copy_from_slice(&VSOCK_MAGIC_BYTES);
    header_bytes[4..8].copy_from_slice(&(MAX_VSOCK_TOTAL_PAYLOAD_SIZE as u32 + 1).to_be_bytes());
    let result = MessageHeader::from_bytes(&header_bytes);
    assert!(result.is_err());
}
