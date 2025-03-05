use crate::error::FairxEnclaveError;
use anyhow::{anyhow, Result};
use kmstool_enclave_lib::{
    kmstool_decrypt_params, kmstool_enclave_decrypt, kmstool_enclave_encrypt, kmstool_enclave_init,
    kmstool_encrypt_params, kmstool_init_params, RESULT_ENCLAVE_KMS_SUCCESS,
};
use log::{error, info};
use std::{ slice};
use std::ffi::CString;
use std::ptr;
use std::thread::sleep;
use typed_builder::TypedBuilder;

#[derive(Debug, Clone, TypedBuilder)]
pub struct KmsEncryptRequest {
    pub plaintext: Vec<u8>,
}

#[derive(Debug, TypedBuilder)]
pub struct KmsEncryptResponse {
    pub ciphertext: Vec<u8>,
}

#[derive(Debug, Clone, TypedBuilder)]
pub struct KmsDecryptRequest {
    pub ciphertext: Vec<u8>,
}

#[derive(Debug, TypedBuilder)]
pub struct KmsDecryptResponse {
    pub plaintext: Vec<u8>,
}

#[derive(Debug, Default, TypedBuilder)]
pub struct KmsConfigure {
    pub aws_region: String,
    pub proxy_port: u32,
    pub aws_access_key_id: String,
    pub aws_secret_access_key: String,
    pub aws_session_token: String,
    pub kms_key_id: String,
    pub kms_encryption_algorithm: String,
    pub with_logs: bool,
}

#[derive(Debug, TypedBuilder)]
pub struct KmsHandler {
    pub config: KmsConfigure,
}

impl KmsHandler {
    pub fn new(config: KmsConfigure) -> Result<Self, FairxEnclaveError> {
        let region = create_cstring(config.aws_region.as_str())?;
        let aws_access_key_id = create_cstring(config.aws_access_key_id.as_str())?;
        let aws_secret_access_key = create_cstring(config.aws_secret_access_key.as_str())?;
        let aws_session_token = create_cstring(config.aws_session_token.as_str())?;
        let key_id = create_cstring(config.kms_key_id.as_str())?;
        let encryption_algorithm = create_cstring("SYMMETRIC_DEFAULT")?;

        let params = kmstool_init_params {
            region: region.as_ptr(),
            proxy_port: config.proxy_port,
            aws_access_key_id: aws_access_key_id.as_ptr(),
            aws_secret_access_key: aws_secret_access_key.as_ptr(),
            aws_session_token: aws_session_token.as_ptr(),
            key_id: key_id.as_ptr(),
            encryption_algorithm: encryption_algorithm.as_ptr(),
            with_logs: if config.with_logs { 1 } else { 0 },
        };

        let rc = unsafe { kmstool_enclave_init(&params) };

        if rc != RESULT_ENCLAVE_KMS_SUCCESS {
            error!("kms tool enclave init error rc: {}", rc);
            return Err(FairxEnclaveError::AnyhowError(anyhow!(
                "kmstool init failed with error code {}",
                rc
            )));
        }
        Ok(Self::builder().config(config).build())
    }

    pub fn encrypt(&self, request: KmsEncryptRequest) -> Result<KmsEncryptResponse, FairxEnclaveError> {
        let params = kmstool_encrypt_params {
            plaintext:request.plaintext.as_ptr(),
            plaintext_len: request.plaintext.len(),
        };
        let mut ciphertext_out: *mut u8 = ptr::null_mut();
        let mut ciphertext_out_len: usize = 0;
        let rc = unsafe { kmstool_enclave_encrypt(&params, &mut ciphertext_out, &mut ciphertext_out_len) };

        if rc != RESULT_ENCLAVE_KMS_SUCCESS {
            error!("kms tool enclave encrypt error rc: {}", rc);
            return Err(FairxEnclaveError::AnyhowError(anyhow!(
                "kmstool encrypt with error code {}",
                rc
            )));
        }

        if ciphertext_out.is_null() {
            return Err(FairxEnclaveError::AnyhowError(anyhow!("Encryption output is null")));
        }

        let ciphertext = unsafe{slice::from_raw_parts(ciphertext_out, ciphertext_out_len).to_vec()};
        unsafe{libc::free(ciphertext_out as *mut libc::c_void);}

        Ok(KmsEncryptResponse::builder().ciphertext(ciphertext).build())
    }

    pub fn decrypt(&self, request: KmsDecryptRequest) -> Result<KmsDecryptResponse, FairxEnclaveError> {
        let params = kmstool_decrypt_params {
            ciphertext:request.ciphertext.as_ptr(),
            ciphertext_len: request.ciphertext.len(),
        };

        let mut plaintext_out: *mut u8 = ptr::null_mut();
        let mut plaintext_out_len: usize = 0;

        let rc = unsafe { kmstool_enclave_decrypt(&params, &mut plaintext_out,&mut plaintext_out_len) };

        if rc != RESULT_ENCLAVE_KMS_SUCCESS {
            error!("kms tool enclave decrypt error rc: {}", rc);
            return Err(FairxEnclaveError::AnyhowError(anyhow!(
                "kmstool decrypt with error code {}",
                rc
            )));
        }

        if plaintext_out.is_null() {
            return Err(FairxEnclaveError::AnyhowError(anyhow!("Decryption output is null")));
        }
        let plaintext = unsafe{slice::from_raw_parts(plaintext_out, plaintext_out_len).to_vec()};
        unsafe{libc::free(plaintext_out as *mut libc::c_void)};
        Ok(KmsDecryptResponse::builder().plaintext(plaintext).build())
    }
}

pub fn create_kms_handler() -> Result<(), FairxEnclaveError> {
    let config = KmsConfigure::builder()
        .proxy_port(8000)
        .aws_region("".to_string())
        .aws_access_key_id("".to_string())
        .aws_secret_access_key("".to_string())
        .aws_session_token("".to_string())
        .kms_key_id("".to_string())
        .kms_encryption_algorithm("SYMMETRIC_DEFAULT".to_string())
        .with_logs(false)
        .build();
    let handler = KmsHandler::new(config).unwrap();

    for i in 1..2000u32 {
        let pk = b"0x30c66137bbd91bb4de8159";
        let mut pk = pk.to_vec();
        pk.extend_from_slice(&i.to_be_bytes());
        let request_enc = KmsEncryptRequest::builder().plaintext(pk.clone()).build();
        let response_enc = handler.encrypt(request_enc)?;
        let request_dec = KmsDecryptRequest::builder().ciphertext(response_enc.ciphertext).build();
        let response_dec = handler.decrypt(request_dec)?;
        assert_eq!(response_dec.plaintext, pk);
        sleep(std::time::Duration::from_secs(2));
    }
    Ok(())
}

fn create_cstring(s: &str) -> Result<CString, FairxEnclaveError> {
    CString::new(s).map_err(|_| FairxEnclaveError::AnyhowError(anyhow!("Failed to create CString")))
}
