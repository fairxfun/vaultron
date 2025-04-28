use super::{cluster::ClusterMessageHandler, internal::InternalMessageHandler};
use crate::{common::EnclaveError, server::EnclaveServerContext};
use enclave_protos::vaultron::enclave::v1::{enclave_request, EnclaveRequest, EnclaveResponse};
use enclave_vsock::VsockMessageHandlerTrait;
use log::warn;
use prost::Message;
use std::sync::Arc;
use typed_builder::TypedBuilder;

#[derive(TypedBuilder)]
pub struct MessageHandler {
    pub internal_handler: Arc<InternalMessageHandler>,
    pub cluster_handler: Arc<ClusterMessageHandler>,
}

impl MessageHandler {
    fn new(context: Arc<EnclaveServerContext>) -> Self {
        let cluster_handler = Arc::new(ClusterMessageHandler::new());
        let internal_handler = Arc::new(InternalMessageHandler::new(context.clone(), cluster_handler.clone()));
        Self::builder()
            .internal_handler(internal_handler)
            .cluster_handler(cluster_handler)
            .build()
    }
}

#[async_trait::async_trait]
impl VsockMessageHandlerTrait for MessageHandler {
    async fn process_request(&self, message: &[u8]) -> Vec<u8> {
        let r = EnclaveRequest::decode(message);
        match r {
            Ok(r) => {
                let response = match r.request {
                    Some(enclave_request::Request::InternalRequest(request)) => {
                        self.internal_handler.process_request(&request).await
                    }
                    Some(enclave_request::Request::ClusterRequest(request)) => {
                        self.cluster_handler.process_request(&request).await
                    }
                    _ => {
                        warn!("Unknown message type");
                        EnclaveResponse::enclave_error(EnclaveError::InvalidRequestError)
                    }
                };
                response.encode_to_vec()
            }
            Err(err) => {
                warn!("Error decoding message: {}", err);
                EnclaveResponse::enclave_error(EnclaveError::InvalidRequestError).encode_to_vec()
            }
        }
    }
}

pub fn create_message_handler(context: Arc<EnclaveServerContext>) -> MessageHandler {
    MessageHandler::new(context)
}
