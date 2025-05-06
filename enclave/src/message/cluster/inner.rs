use crate::cluster::EnclaveClusterKeys;
use crate::server::EnclaveServerContext;
use crate::EnclaveError;
use anyhow::Result;
use enclave_protos::vaultron::{
    enclave::attestation::v1::AttestationDocumentUserData,
    enclave::common::v1::EnclaveType,
    enclave::v1::{enclave_cluster_request, EnclaveClusterRequest, EnclaveResponse},
};
use prost::Message;
use serde::Serialize;
use std::sync::Arc;
use typed_builder::TypedBuilder;

#[derive(TypedBuilder)]
pub struct ClusterMessageHandlerInner {
    pub context: Arc<EnclaveServerContext>,
    pub cluster_type: EnclaveType,
    pub cluster_key: Arc<EnclaveClusterKeys>,
}

impl ClusterMessageHandlerInner {
    pub fn new(
        context: Arc<EnclaveServerContext>,
        seed: &[u8],
        enclave_type: EnclaveType,
    ) -> Result<Self, EnclaveError> {
        let cluster_key = EnclaveClusterKeys::new(seed)?;
        Ok(Self::builder()
            .context(context)
            .cluster_type(enclave_type)
            .cluster_key(Arc::new(cluster_key))
            .build())
    }

    pub async fn process_request(&self, r: &EnclaveClusterRequest) -> EnclaveResponse {
        if self.cluster_type == EnclaveType::Seed {
            return EnclaveResponse::enclave_error(EnclaveError::SeedCannotProcessRequest);
        }

        match &r.request {
            Some(enclave_cluster_request::Request::CreateUserWalletRequest(request)) => {
                let result = self.handle_create_user_wallet_request(request).await;
                match result {
                    Ok(response) => self.build_response(request, &response),
                    Err(err) => EnclaveResponse::enclave_error(err),
                }
            }
            _ => EnclaveResponse::enclave_error(EnclaveError::InvalidRequestError),
        }
    }

    pub fn get_cluster_public_key(&self) -> Vec<u8> {
        self.cluster_key.get_cluster_public_key()
    }

    pub fn get_encoded_cluster_seed(&self, public_key: Vec<u8>) -> Result<Vec<u8>, EnclaveError> {
        self.cluster_key.encrypt_seed(public_key)
    }

    fn build_response<R, P>(&self, request: &R, response: &P) -> EnclaveResponse
    where
        R: Message + Serialize,
        P: Message + Serialize,
    {
        let public_key = self.get_cluster_public_key();
        let user_data = AttestationDocumentUserData::encode_message(request, response);
        match self
            .context
            .nsm_handle
            .generate_attestation_document(user_data, public_key)
        {
            Ok(doc) => EnclaveResponse::success(doc),
            Err(err) => EnclaveResponse::enclave_error(err),
        }
    }
}
