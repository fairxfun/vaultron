use super::InternalMessageHandler;
use crate::common::EnclaveError;
use enclave_protos::vaultron::enclave::internal::v1::{PingRequest, PingResponse};

impl InternalMessageHandler {
    pub(crate) async fn handle_ping_request(&self, _request: &PingRequest) -> Result<PingResponse, EnclaveError> {
        Ok(PingResponse::builder().build())
    }
}
