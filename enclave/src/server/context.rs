use crate::{
    common::{EnclaveError, EnclaveSettings},
    nsm::EnclaveNsmHandle,
};
use anyhow::Result;
use enclave_attestation::AttestationParser;
use std::sync::Arc;
use typed_builder::TypedBuilder;

#[derive(TypedBuilder)]
pub struct EnclaveServerContext {
    pub settings: Arc<EnclaveSettings>,
    pub nsm_handle: Arc<EnclaveNsmHandle>,
    pub attestation_parser: Arc<AttestationParser>,
}

impl EnclaveServerContext {
    pub async fn new() -> Result<Self, EnclaveError> {
        let attestation_parser = Arc::new(AttestationParser::new(None, None).await?);
        let nsm_handle = Arc::new(EnclaveNsmHandle::new());
        let (locked, pcr0) = nsm_handle.get_pcr(0u16)?;
        if !locked {
            return Err(EnclaveError::NsmApiError);
        }
        let seed = nsm_handle.get_random_bytes(32)?;
        let settings = Arc::new(EnclaveSettings::new(pcr0, seed)?);
        Ok(EnclaveServerContext::builder()
            .attestation_parser(attestation_parser)
            .settings(settings)
            .nsm_handle(nsm_handle)
            .build())
    }
}
