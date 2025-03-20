use crate::gen::{
    kmstool_decrypt_params, kmstool_enclave_decrypt, kmstool_enclave_encrypt, kmstool_enclave_init,
    kmstool_encrypt_params, kmstool_init_params, KMSTOOL_STATUS,
};
use crate::gen::{kmstool_enclave_update_aws_key, kmstool_update_aws_key_params};
use crate::{
    EnclaveKmstoolError, KmsToolTrait, KmstoolDecryptParams, KmstoolDecryptResult, KmstoolEncryptParams,
    KmstoolEncryptResult, KmstoolInitParams, KmstoolUpdateAwsCredentialsParams,
};
use anyhow::anyhow;
use log::error;
use std::ffi::CString;
use std::{ptr, slice};
use typed_builder::TypedBuilder;

#[derive(Debug, TypedBuilder, Default)]
pub(crate) struct KmsToolCLibClient {}

impl KmsToolTrait for KmsToolCLibClient {
    fn init(&self, params: KmstoolInitParams) -> anyhow::Result<(), EnclaveKmstoolError> {
        let aws_region = create_cstring(params.aws_region.as_str())?;

        let params = kmstool_init_params {
            aws_region: aws_region.as_ptr(),
            proxy_port: params.proxy_port,
            enable_logging: if params.enable_logging { 1 } else { 0 },
        };

        let rc = unsafe { kmstool_enclave_init(&params) };

        if rc != KMSTOOL_STATUS::KMSTOOL_SUCCESS as i32 {
            error!("kms tool enclave init error rc: {}", rc);
            return Err(EnclaveKmstoolError::AnyhowError(anyhow!(
                "kmstool init failed with error code {}",
                rc
            )));
        }

        Ok(())
    }

    fn update_aws_credentials(
        &self,
        request: KmstoolUpdateAwsCredentialsParams,
    ) -> anyhow::Result<(), EnclaveKmstoolError> {
        let aws_access_key_id = create_cstring(request.aws_access_key_id.as_str())?;
        let aws_secret_access_key = create_cstring(request.aws_secret_access_key.as_str())?;
        let aws_session_token = create_cstring(request.aws_session_token.as_str())?;

        let params = kmstool_update_aws_key_params {
            aws_access_key_id: aws_access_key_id.as_ptr(),
            aws_secret_access_key: aws_secret_access_key.as_ptr(),
            aws_session_token: aws_session_token.as_ptr(),
        };

        let rc = unsafe { kmstool_enclave_update_aws_key(&params) };

        if rc != KMSTOOL_STATUS::KMSTOOL_SUCCESS as i32 {
            error!("kms tool enclave update aws credentials error rc: {}", rc);
            return Err(EnclaveKmstoolError::AnyhowError(anyhow!(
                "kmstool update aws credentials with error code {}",
                rc
            )));
        }

        Ok(())
    }

    fn encrypt(&self, request: KmstoolEncryptParams) -> anyhow::Result<KmstoolEncryptResult, EnclaveKmstoolError> {
        let kms_key_id = create_cstring(request.kms_key_id.as_str())?;
        let params = kmstool_encrypt_params {
            kms_key_id: kms_key_id.as_ptr(),
            plaintext: request.plaintext.as_ptr(),
            plaintext_len: request.plaintext.len() as u32,
        };
        let mut ciphertext_out: *mut u8 = ptr::null_mut();
        let mut ciphertext_out_len: u32 = 0;
        let rc = unsafe { kmstool_enclave_encrypt(&params, &mut ciphertext_out, &mut ciphertext_out_len) };

        if rc != KMSTOOL_STATUS::KMSTOOL_SUCCESS as i32 {
            error!("kms tool enclave encrypt error rc: {}", rc);
            return Err(EnclaveKmstoolError::AnyhowError(anyhow!(
                "kmstool encrypt with error code {}",
                rc
            )));
        }

        if ciphertext_out.is_null() {
            return Err(EnclaveKmstoolError::AnyhowError(anyhow!("Encryption output is null")));
        }

        let ciphertext = unsafe { slice::from_raw_parts(ciphertext_out, ciphertext_out_len as usize).to_vec() };
        unsafe {
            libc::free(ciphertext_out as *mut libc::c_void);
        }

        Ok(KmstoolEncryptResult::builder().ciphertext(ciphertext).build())
    }

    fn decrypt(&self, request: KmstoolDecryptParams) -> anyhow::Result<KmstoolDecryptResult, EnclaveKmstoolError> {
        let kms_key_id = create_cstring(request.kms_key_id.as_str())?;
        let kms_algorithm = create_cstring(request.kms_algorithm.as_str())?;
        let params = kmstool_decrypt_params {
            kms_key_id: kms_key_id.as_ptr(),
            kms_algorithm: kms_algorithm.as_ptr(),
            ciphertext: request.ciphertext.as_ptr(),
            ciphertext_len: request.ciphertext.len() as u32,
        };

        let mut plaintext_out: *mut u8 = ptr::null_mut();
        let mut plaintext_out_len: u32 = 0;

        let rc = unsafe { kmstool_enclave_decrypt(&params, &mut plaintext_out, &mut plaintext_out_len) };

        if rc != KMSTOOL_STATUS::KMSTOOL_SUCCESS as i32 {
            error!("kms tool enclave decrypt error rc: {}", rc);
            return Err(EnclaveKmstoolError::AnyhowError(anyhow!(
                "kmstool decrypt with error code {}",
                rc
            )));
        }

        if plaintext_out.is_null() {
            return Err(EnclaveKmstoolError::AnyhowError(anyhow!("Decryption output is null")));
        }
        let plaintext = unsafe { slice::from_raw_parts(plaintext_out, plaintext_out_len as usize).to_vec() };
        unsafe { libc::free(plaintext_out as *mut libc::c_void) };
        Ok(KmstoolDecryptResult::builder().plaintext(plaintext).build())
    }
}

fn create_cstring(s: &str) -> anyhow::Result<CString, EnclaveKmstoolError> {
    CString::new(s).map_err(|_| EnclaveKmstoolError::AnyhowError(anyhow!("Failed to create CString")))
}
