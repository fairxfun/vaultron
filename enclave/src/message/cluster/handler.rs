use super::inner::ClusterMessageHandlerInner;
use crate::common::EnclaveError;
use crate::EnclaveServerContext;
use enclave_protos::vaultron::enclave::common::v1::EnclaveType;
use enclave_protos::vaultron::enclave::v1::{EnclaveClusterRequest, EnclaveResponse};
use log::info;
use std::sync::Arc;
use tokio::sync::RwLock;
use typed_builder::TypedBuilder;

#[derive(TypedBuilder)]
pub struct ClusterMessageHandler {
    pub handler: RwLock<Option<Arc<ClusterMessageHandlerInner>>>,
}

impl ClusterMessageHandler {
    pub fn new() -> Self {
        Self::builder().handler(RwLock::new(None)).build()
    }

    pub async fn initialize(
        &self,
        context: Arc<EnclaveServerContext>,
        seed: &[u8],
        enclave_type: EnclaveType,
    ) -> Result<Vec<u8>, EnclaveError> {
        if self.is_handler_initialized().await {
            return Err(EnclaveError::ClusterAlreadyInitialized);
        }

        let handler = ClusterMessageHandlerInner::new(context, seed, enclave_type)?;
        let public_key = handler.get_cluster_public_key();
        info!(
            "Enclave start with type: {:?}, cluster public key: {:?}",
            enclave_type, public_key
        );
        let mut write = self.handler.write().await;
        *write = Some(Arc::new(handler));
        Ok(public_key)
    }

    pub async fn process_request(&self, request: &EnclaveClusterRequest) -> EnclaveResponse {
        let handler = self.get_message_handler().await;
        match handler {
            Ok(handler) => handler.process_request(request).await,
            Err(err) => EnclaveResponse::error(err),
        }
    }

    pub async fn get_cluster_type(&self) -> Result<EnclaveType, EnclaveError> {
        let handler = self.get_message_handler().await?;
        Ok(handler.cluster_type)
    }

    pub async fn get_cluster_public_key(&self) -> Result<Vec<u8>, EnclaveError> {
        let handler = self.get_message_handler().await?;
        Ok(handler.get_cluster_public_key())
    }

    pub async fn get_encoded_cluster_seed(&self, public_key: Vec<u8>) -> Result<Vec<u8>, EnclaveError> {
        let cluster_handler = self.get_message_handler().await?;
        cluster_handler.get_encoded_cluster_seed(public_key)
    }

    pub async fn is_handler_initialized(&self) -> bool {
        let read = self.handler.read().await;
        read.as_ref().is_some()
    }

    async fn get_message_handler(&self) -> Result<Arc<ClusterMessageHandlerInner>, EnclaveError> {
        let read = self.handler.read().await;
        match read.as_ref() {
            Some(handler) => Ok(handler.clone()),
            None => Err(EnclaveError::ClusterNotInitialized),
        }
    }
}
