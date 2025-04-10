use super::InternalMessageHandler;
use crate::common::EnclaveError;
use enclave_protos::vaultron::internal::v1::{PingRequest, PingResponse};
use log::info;

impl InternalMessageHandler {
    pub(crate) async fn handle_ping_request(&self, _request: &PingRequest) -> Result<PingResponse, EnclaveError> {
        info!("Received ping request");
        Ok(PingResponse::builder().build())
    }
}
