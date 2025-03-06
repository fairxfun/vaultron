use crate::error::EnclaveError;
use enclave_kmstool::{create_kmstool_clib_client, KmsDecryptRequest, KmsEncryptRequest, KmsInitRequest, KmsToolTrait};
use log::info;
use std::thread::sleep;

pub fn create_kms_handler() -> anyhow::Result<(), EnclaveError> {
    let kms_tool = create_kmstool_clib_client();
    let init_params = KmsInitRequest::builder()
        .proxy_port(8000)
        .aws_region("".to_string())
        .aws_access_key_id("".to_string())
        .aws_secret_access_key("".to_string())
        .aws_session_token("".to_string())
        .kms_key_id("60b8ce3a-7466-42b7-96a7-a3868f0fd1bf".to_string())
        .kms_encryption_algorithm("SYMMETRIC_DEFAULT".to_string())
        .enable_logging(false)
        .build();
    kms_tool.init(init_params)?;

    for i in 1..2000u32 {
        info!("round i {}", i);
        let pk = b"0x30c66137bbd91bb4de8159";
        let mut pk = pk.to_vec();
        pk.extend_from_slice(&i.to_be_bytes());
        let request_enc = KmsEncryptRequest::builder().plaintext(pk.clone()).build();
        let response_enc = kms_tool.encrypt(request_enc)?;
        let request_dec = KmsDecryptRequest::builder().ciphertext(response_enc.ciphertext).build();
        let response_dec = kms_tool.decrypt(request_dec)?;
        assert_eq!(response_dec.plaintext, pk);
        sleep(std::time::Duration::from_secs(2));
    }
    Ok(())
}
