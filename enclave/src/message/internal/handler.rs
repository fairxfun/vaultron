use crate::{message::cluster::ClusterMessageHandler, server::EnclaveServerContext, EnclaveError};
use enclave_protos::vaultron::{
    enclave::attestation::v1::AttestationDocumentUserData,
    enclave::v1::{enclave_internal_request, EnclaveInternalRequest, EnclaveResponse, EnclaveResponsePadding},
};
use prost::Message;
use serde::Serialize;
use std::sync::Arc;
use typed_builder::TypedBuilder;

#[derive(TypedBuilder)]
pub struct InternalMessageHandler {
    pub context: Arc<EnclaveServerContext>,
    pub cluster_handler: Arc<ClusterMessageHandler>,
}

impl InternalMessageHandler {
    pub fn new(context: Arc<EnclaveServerContext>, cluster_handler: Arc<ClusterMessageHandler>) -> Self {
        Self::builder()
            .context(context)
            .cluster_handler(cluster_handler)
            .build()
    }

    pub async fn process_request(&self, r: &EnclaveInternalRequest) -> EnclaveResponse {
        match &r.request {
            Some(enclave_internal_request::Request::PingRequest(request)) => {
                let result = self.handle_ping_request(request).await;
                match result {
                    Ok(response) => self.build_response(request, &response),
                    Err(err) => EnclaveResponse::error(err),
                }
            }
            Some(enclave_internal_request::Request::GetAttributesRequest(request)) => {
                let result = self.handle_get_attributes_request(request).await;
                match result {
                    Ok(response) => self.build_response(request, &response),
                    Err(err) => EnclaveResponse::error(err),
                }
            }
            Some(enclave_internal_request::Request::InitClusterKeyRequest(request)) => {
                let result = self.handle_init_cluster_key_request(request).await;
                match result {
                    Ok(response) => self.build_response(request, &response),
                    Err(err) => EnclaveResponse::error(err),
                }
            }
            Some(enclave_internal_request::Request::InitClusterKeySyncRequest(request)) => {
                let result = self.handle_init_cluster_key_sync_request(request).await;
                match result {
                    Ok(response) => self.build_response(request, &response),
                    Err(err) => EnclaveResponse::error(err),
                }
            }
            Some(enclave_internal_request::Request::ForwardClusterKeySyncRequest(request)) => {
                let result = self.handle_forward_cluster_key_sync_request(request).await;
                match result {
                    Ok(response) => self.build_padding_response(&response),
                    Err(err) => EnclaveResponse::error(err),
                }
            }
            Some(enclave_internal_request::Request::ReplyClusterKeySyncRequest(request)) => {
                let result = self.handle_reply_cluster_key_sync_request(request).await;
                match result {
                    Ok(response) => self.build_padding_response(&response),
                    Err(err) => EnclaveResponse::error(err),
                }
            }
            _ => EnclaveResponse::error(EnclaveError::InvalidRequestError),
        }
    }

    fn build_padding_response<P>(&self, response: &P) -> EnclaveResponse
    where
        P: Message + Serialize,
    {
        let request = EnclaveResponsePadding {};
        self.build_response(&request, response)
    }

    fn build_response<R, P>(&self, request: &R, response: &P) -> EnclaveResponse
    where
        R: Message + Serialize,
        P: Message + Serialize,
    {
        let public_key = self.context.settings.local_key.public_key().to_vec();
        let user_data = AttestationDocumentUserData::encode_message(request, response);
        match self
            .context
            .nsm_handle
            .generate_attestation_document(user_data, public_key)
        {
            Ok(attestation_doc) => EnclaveResponse::success(attestation_doc),
            Err(err) => EnclaveResponse::error(err),
        }
    }
}
