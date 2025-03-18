use super::MessageHandler;
use crate::common::EnclaveError;
use enclave_protos::enclave::v1::{PingRequest, PingResponse};
use log::info;

impl MessageHandler {
    pub async fn handle_ping_request(&self, _request: PingRequest) -> Result<PingResponse, EnclaveError> {
        info!("Received ping request");
        Ok(PingResponse::success())
    }
}
