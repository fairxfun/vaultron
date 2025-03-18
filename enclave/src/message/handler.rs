use std::sync::Arc;

use crate::{common::EnclaveError, server::EnclaveServerContext};
use enclave_protos::enclave::v1::{
    enclave_request, enclave_response, CreateEnclaveWalletResponse, EnclaveRequest, EnclaveResponse,
    InitKmstoolResponse, PingResponse, UpdateAwsCredentialsResponse,
};
use enclave_vsock::VsockMessageHandlerTrait;
use log::warn;
use prost::Message;
use typed_builder::TypedBuilder;

#[derive(Debug, TypedBuilder)]
pub struct MessageHandler {
    pub context: Arc<EnclaveServerContext>,
}

impl MessageHandler {
    fn new(context: Arc<EnclaveServerContext>) -> Self {
        Self::builder().context(context).build()
    }
}

#[async_trait::async_trait]
impl VsockMessageHandlerTrait for MessageHandler {
    type Error = EnclaveError;

    async fn process_message(&self, message: &[u8]) -> Result<Vec<u8>, EnclaveError> {
        let mr = EnclaveRequest::decode(message)?;
        match mr.request {
            Some(enclave_request::Request::PingRequest(r)) => {
                let result = self.handle_ping_request(r).await;
                let response = match result {
                    Ok(r) => r,
                    Err(e) => PingResponse::error(e),
                };
                let response = EnclaveResponse::builder()
                    .response(enclave_response::Response::PingResponse(response))
                    .build();
                Ok(response.encode_to_vec())
            }
            Some(enclave_request::Request::InitKmstoolRequest(r)) => {
                let result = self.handle_kmstool_init_request(r).await;
                let response = match result {
                    Ok(r) => r,
                    Err(e) => InitKmstoolResponse::error(e),
                };
                let response = EnclaveResponse::builder()
                    .response(enclave_response::Response::InitKmstoolResponse(response))
                    .build();
                Ok(response.encode_to_vec())
            }
            Some(enclave_request::Request::UpdateAwsCredentialsRequest(r)) => {
                let result = self.handle_update_aws_credentials_request(r).await;
                let response = match result {
                    Ok(r) => r,
                    Err(e) => UpdateAwsCredentialsResponse::error(e),
                };
                let response = EnclaveResponse::builder()
                    .response(enclave_response::Response::UpdateAwsCredentialsResponse(response))
                    .build();
                Ok(response.encode_to_vec())
            }
            Some(enclave_request::Request::CreateEnclaveWalletRequest(r)) => {
                let result = self.handle_create_enclave_wallet_request(r).await;
                let response = match result {
                    Ok(r) => r,
                    Err(e) => CreateEnclaveWalletResponse::error(e),
                };
                let response = EnclaveResponse::builder()
                    .response(enclave_response::Response::CreateEnclaveWalletResponse(response))
                    .build();
                Ok(response.encode_to_vec())
            }
            _ => {
                warn!("Unknown message type: {:?}", message);
                Err(EnclaveError::InvalidRequestError)
            }
        }
    }
}

pub fn create_message_handler(context: Arc<EnclaveServerContext>) -> MessageHandler {
    MessageHandler::new(context)
}
