use crate::gen::{
    kmstool_decrypt_params, kmstool_enclave_decrypt, kmstool_enclave_encrypt, kmstool_enclave_get_attestation_document,
    kmstool_enclave_get_key_policy, kmstool_enclave_init, kmstool_encrypt_params, kmstool_init_params, KMSTOOL_STATUS,
};
use crate::gen::{
    kmstool_enclave_list_key_policies, kmstool_enclave_update_aws_key, kmstool_get_key_policy_params,
    kmstool_list_key_policies_params, kmstool_update_aws_key_params,
};
use crate::{
    EnclaveKmstoolError, KmsToolTrait, KmstoolDecryptParams, KmstoolDecryptResult, KmstoolEncryptParams,
    KmstoolEncryptResult, KmstoolGetAttestationDocumentResult, KmstoolGetKeyPolicyParams, KmstoolGetKeyPolicyResult,
    KmstoolInitParams, KmstoolListKeyPoliciesParams, KmstoolListKeyPoliciesResult, KmstoolUpdateAwsCredentialsParams,
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

    fn get_attestation_document(&self) -> anyhow::Result<KmstoolGetAttestationDocumentResult, EnclaveKmstoolError> {
        let mut out: *mut u8 = ptr::null_mut();
        let mut out_len: u32 = 0;

        let rc = unsafe { kmstool_enclave_get_attestation_document(&mut out_len, &mut out) };
        if rc != KMSTOOL_STATUS::KMSTOOL_SUCCESS as i32 {
            error!("kms tool enclave get attestation document error rc: {}", rc);
            return Err(EnclaveKmstoolError::AnyhowError(anyhow!(
                "kmstool get attestation document with error code {}",
                rc
            )));
        }

        let attestation_document = unsafe { slice::from_raw_parts(out, out_len as usize).to_vec() };
        unsafe { libc::free(out as *mut libc::c_void) };

        Ok(KmstoolGetAttestationDocumentResult::builder()
            .attestation_document(attestation_document)
            .build())
    }

    fn list_key_policies(
        &self,
        request: KmstoolListKeyPoliciesParams,
    ) -> anyhow::Result<KmstoolListKeyPoliciesResult, EnclaveKmstoolError> {
        let key_id = create_cstring(request.key_id.as_str())?;
        let limit = request.limit.unwrap_or(100);

        let (_marker_cstring, marker_ptr) = match request.marker {
            Some(ref marker) => {
                let cstring = create_cstring(marker.as_str())?;
                let ptr = cstring.as_ptr();
                (Some(cstring), ptr)
            }
            None => (None, ptr::null()),
        };
        let params = kmstool_list_key_policies_params {
            key_id: key_id.as_ptr(),
            limit,
            marker: marker_ptr,
        };

        let mut policies_out: *mut u8 = ptr::null_mut();
        let mut policies_out_len: u32 = 0;

        let rc = unsafe { kmstool_enclave_list_key_policies(&params, &mut policies_out_len, &mut policies_out) };

        if rc != KMSTOOL_STATUS::KMSTOOL_SUCCESS as i32 {
            error!("kms tool enclave list key policies error rc: {}", rc);
            return Err(EnclaveKmstoolError::AnyhowError(anyhow!(
                "kmstool list key policies with error code {}",
                rc
            )));
        }

        let policies = unsafe { slice::from_raw_parts(policies_out, policies_out_len as usize).to_vec() };
        unsafe { libc::free(policies_out as *mut libc::c_void) };

        Ok(KmstoolListKeyPoliciesResult::builder().policies(policies).build())
    }

    fn get_key_policy(
        &self,
        request: KmstoolGetKeyPolicyParams,
    ) -> anyhow::Result<KmstoolGetKeyPolicyResult, EnclaveKmstoolError> {
        let key_id = create_cstring(request.key_id.as_str())?;
        let policy_name = create_cstring(request.policy_name.as_str())?;

        let params = kmstool_get_key_policy_params {
            key_id: key_id.as_ptr(),
            policy_name: policy_name.as_ptr(),
        };

        let mut policy_out: *mut u8 = ptr::null_mut();
        let mut policy_out_len: u32 = 0;

        let rc = unsafe { kmstool_enclave_get_key_policy(&params, &mut policy_out_len, &mut policy_out) };

        if rc != KMSTOOL_STATUS::KMSTOOL_SUCCESS as i32 {
            error!("kms tool enclave get key policy error rc: {}", rc);
            return Err(EnclaveKmstoolError::AnyhowError(anyhow!(
                "kmstool get key policy with error code {}",
                rc
            )));
        }

        let policy = unsafe { slice::from_raw_parts(policy_out, policy_out_len as usize).to_vec() };
        unsafe { libc::free(policy_out as *mut libc::c_void) };

        Ok(KmstoolGetKeyPolicyResult::builder().policy(policy).build())
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
        let rc = unsafe { kmstool_enclave_encrypt(&params, &mut ciphertext_out_len, &mut ciphertext_out) };

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

        let rc = unsafe { kmstool_enclave_decrypt(&params, &mut plaintext_out_len, &mut plaintext_out) };

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
