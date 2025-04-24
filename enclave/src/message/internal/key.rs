use super::InternalMessageHandler;
use crate::common::EnclaveError;
use enclave_crypto::ENCRYPTION_KEY_LENGTH;
use enclave_protos::vaultron::enclave::common::v1::EnclaveType;
use enclave_protos::vaultron::enclave::internal::v1::{
    ForwardClusterKeySyncRequest, ForwardClusterKeySyncResponse, InitClusterKeyRequest, InitClusterKeyResponse,
    InitClusterKeySyncRequest, InitClusterKeySyncResponse, ReplyClusterKeySyncRequest, ReplyClusterKeySyncResponse,
};
use log::info;

impl InternalMessageHandler {
    pub(crate) async fn handle_init_cluster_key_request(
        &self,
        _request: &InitClusterKeyRequest,
    ) -> Result<InitClusterKeyResponse, EnclaveError> {
        info!("Received init cluster key request");
        if self.cluster_handler.is_handler_initialized().await {
            return Err(EnclaveError::ClusterAlreadyInitialized);
        }
        let cluster_seed = self.context.nsm_handle.get_random_bytes(ENCRYPTION_KEY_LENGTH)?;
        let cluster_public_key = self
            .cluster_handler
            .initialize(self.context.clone(), &cluster_seed, EnclaveType::Seed)
            .await?;
        Ok(InitClusterKeyResponse::builder()
            .cluster_public_key(cluster_public_key)
            .build())
    }

    pub(crate) async fn handle_init_cluster_key_sync_request(
        &self,
        _request: &InitClusterKeySyncRequest,
    ) -> Result<InitClusterKeySyncResponse, EnclaveError> {
        info!("Received init cluster key sync request");
        if self.cluster_handler.is_handler_initialized().await {
            return Err(EnclaveError::ClusterAlreadyInitialized);
        }
        Ok(InitClusterKeySyncResponse::builder().build())
    }

    pub(crate) async fn handle_forward_cluster_key_sync_request(
        &self,
        request: &ForwardClusterKeySyncRequest,
    ) -> Result<ForwardClusterKeySyncResponse, EnclaveError> {
        info!("Received forward cluster key sync request");
        if !self.cluster_handler.is_handler_initialized().await {
            return Err(EnclaveError::ClusterNotInitialized);
        }
        let expected_pcr0 = self.context.settings.pcr0.clone();
        let requester_doc = self
            .context
            .attestation_parser
            .verify_and_parse(&request.requester_doc, &expected_pcr0)?;
        let requester_pubkey = requester_doc.public_key;
        let encrypted_cluster_seed = self.cluster_handler.get_encoded_cluster_seed(requester_pubkey).await?;
        Ok(ForwardClusterKeySyncResponse::builder()
            .encrypted_cluster_seed(encrypted_cluster_seed)
            .build())
    }

    pub(crate) async fn handle_reply_cluster_key_sync_request(
        &self,
        request: &ReplyClusterKeySyncRequest,
    ) -> Result<ReplyClusterKeySyncResponse, EnclaveError> {
        info!("Received reply cluster key sync request");
        if self.cluster_handler.is_handler_initialized().await {
            return Err(EnclaveError::ClusterAlreadyInitialized);
        }
        let expected_pcr0 = self.context.settings.pcr0.clone();
        let responder_doc = self
            .context
            .attestation_parser
            .verify_and_parse(&request.responder_doc, &expected_pcr0)?;
        let (_, responder_response) =
            responder_doc.decode_user_data::<ForwardClusterKeySyncRequest, ForwardClusterKeySyncResponse>()?;
        let cluster_seed = self
            .context
            .settings
            .local_key
            .decrypt_by_private_key(&responder_response.encrypted_cluster_seed)?;
        let enclave_type = EnclaveType::from_i32(request.enclave_type).unwrap_or(EnclaveType::Seed);
        let cluster_public_key = self
            .cluster_handler
            .initialize(self.context.clone(), &cluster_seed, enclave_type)
            .await?;
        if cluster_public_key != responder_response.cluster_public_key {
            return Err(EnclaveError::InvalidClusterPublicKeyError);
        }
        Ok(ReplyClusterKeySyncResponse::builder()
            .cluster_public_key(cluster_public_key)
            .build())
    }
}
