use super::InternalMessageHandler;
use crate::common::EnclaveError;
use enclave_protos::vaultron::internal::v1::{EnclaveData, PingRequest, PingResponse};
use log::info;

impl InternalMessageHandler {
    pub(crate) async fn handle_ping_request(&self, _request: &PingRequest) -> Result<PingResponse, EnclaveError> {
        info!("Received ping request");
        let enclave_data = EnclaveData::builder()
            .enclave_pcr0(self.context.settings.pcr0.clone())
            .enclave_public_key(self.context.settings.local_key.public_key().to_vec())
            .build();
        Ok(PingResponse::builder().enclave_data(enclave_data).build())
    }
}
