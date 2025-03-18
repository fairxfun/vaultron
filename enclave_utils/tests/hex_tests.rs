use enclave_utils::hex::{
    decode_hex, decode_hex_with_length, encode_fixed_len_hex_with_prefix, encode_hex, encode_hex_with_prefix,
};

#[tokio::test]
pub async fn test_hex_encode_decode() {
    let data = vec![0x12, 0x34, 0x56, 0x78, 0x90, 0xab, 0xcd, 0xef];
    let encoded_str = encode_hex(data.clone());
    assert_eq!(encoded_str, "1234567890abcdef");

    let decoded_data = decode_hex(encoded_str).unwrap();
    assert_eq!(decoded_data, data);
}

#[tokio::test]
pub async fn test_hex_encode_decode_with_prefix() {
    let data = vec![0x12, 0x34, 0x56, 0x78, 0x90, 0xab, 0xcd, 0xef];
    let encoded_str = encode_hex_with_prefix(data.clone());
    assert_eq!(encoded_str, "0x1234567890abcdef");

    let decoded_data = decode_hex(encoded_str).unwrap();
    assert_eq!(decoded_data, data);
}

#[tokio::test]
pub async fn test_encode_fixed_len_hex_with_prefix() {
    let data = vec![0x12, 0x34, 0x56, 0x78, 0x90, 0xab, 0xcd, 0xef];
    let encoded_str = encode_fixed_len_hex_with_prefix(data.clone(), 4);
    assert_eq!(encoded_str, "0x12345678");

    let decoded_data = decode_hex_with_length::<4, _>(encoded_str).unwrap();
    assert_eq!(decoded_data, data[0..4]);
}
