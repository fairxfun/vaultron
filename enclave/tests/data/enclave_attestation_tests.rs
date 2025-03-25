use enclave_utils::hex::decode_hex;
use vaultron_enclave::data::decode_attestation_doc;

#[tokio::test]
pub async fn test_decode_attestation_doc() {
    let attestation_data = include_bytes!("../test_files/enclave/attestation_doc");
    let attestation_data = String::from_utf8(attestation_data.to_vec()).unwrap();
    let attestation_doc = decode_hex(&attestation_data).unwrap();
    let attestation_doc = decode_attestation_doc(attestation_doc.as_slice()).unwrap();
    assert_eq!(attestation_doc.pcrs.len(), 16);
    assert_eq!(attestation_doc.module_id, "i-0ff51feda5f8c3609-enc0195c7da7fb846bc");
    assert_eq!(attestation_doc.timestamp, 1742814752346);
}
