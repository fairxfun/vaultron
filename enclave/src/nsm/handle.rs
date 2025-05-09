use super::{AttestationRequest, ATTESTATION_NONCE_SIZE};
use crate::common::EnclaveError;
use anyhow::Result;
use aws_nitro_enclaves_nsm_api::api::{Request, Response};
use aws_nitro_enclaves_nsm_api::driver::{nsm_exit, nsm_init, nsm_process_request};
use enclave_crypto::random_bytes_by_rng;

#[derive(Debug)]
pub struct EnclaveNsmHandle {
    nsm_ctx: i32,
}

impl Drop for EnclaveNsmHandle {
    fn drop(&mut self) {
        self.exit();
    }
}

impl Default for EnclaveNsmHandle {
    fn default() -> Self {
        Self::new()
    }
}

impl EnclaveNsmHandle {
    pub fn new() -> Self {
        let nsm_ctx = nsm_init();
        Self { nsm_ctx }
    }

    pub fn exit(&self) {
        nsm_exit(self.nsm_ctx);
    }
    pub fn get_random_bytes(&self, length: usize) -> Result<Vec<u8>, EnclaveError> {
        let mut random_bytes = Vec::with_capacity(length);
        while random_bytes.len() < length {
            let response = nsm_process_request(self.nsm_ctx, Request::GetRandom);
            match response {
                Response::GetRandom { random } => {
                    let needed = length - random_bytes.len();
                    random_bytes.extend_from_slice(&random[..needed.min(random.len())]);
                }
                _ => return Err(EnclaveError::NsmApiError),
            }
        }
        let random_bytes_rng = random_bytes_by_rng(length);
        let mut combined_random = vec![0u8; length];
        for i in 0..length {
            combined_random[i] = random_bytes[i] ^ random_bytes_rng[i];
        }
        Ok(combined_random)
    }

    pub fn get_pcr(&self, index: u16) -> Result<(bool, Vec<u8>), EnclaveError> {
        let request = Request::DescribePCR { index };
        let response = nsm_process_request(self.nsm_ctx, request);
        match response {
            Response::DescribePCR { lock, data } => Ok((lock, data)),
            _ => Err(EnclaveError::NsmApiError),
        }
    }

    /// Generate a new attestation document with optional user data
    pub fn generate_attestation_document(
        &self,
        user_data: Vec<u8>,
        public_key: Vec<u8>,
    ) -> Result<Vec<u8>, EnclaveError> {
        let nonce = random_bytes_by_rng(ATTESTATION_NONCE_SIZE);
        let request = AttestationRequest::builder()
            .user_data(user_data)
            .nonce(nonce)
            .public_key(public_key)
            .build();
        let response = nsm_process_request(self.nsm_ctx, request.into());
        let document_bytes = match response {
            Response::Attestation { document } => document,
            _ => return Err(EnclaveError::InvalidAttestationDocumentError),
        };
        Ok(document_bytes)
    }
}
