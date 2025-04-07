use enclave_attestation::AttestationParser;
use std::env;

#[tokio::test]
pub async fn test_decode_attestation_doc() {
    let attestation_data = include_bytes!("./test_files/enclave/attestation_doc");
    let current_dir = env::current_dir().unwrap();
    let cert_path = current_dir.join("tests/test_files/certs/aws-nitro-enclaves-root.pem");
    let attestation_parser = AttestationParser::new(Some(cert_path.to_str().unwrap().to_string()), None)
        .await
        .unwrap();
    let attestation_doc = attestation_parser.verify_and_parse(attestation_data, &[0; 32]);
    print!("result {:?}", attestation_doc);
}
