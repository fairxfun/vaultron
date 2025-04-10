use super::InternalMessageHandler;
use crate::common::EnclaveError;
use crate::common::GIT_REVISION;
use enclave_protos::vaultron::internal::v1::{GetAttributesRequest, GetAttributesResponse};
use log::info;

impl InternalMessageHandler {
    pub(crate) async fn handle_get_attributes_request(
        &self,
        _request: &GetAttributesRequest,
    ) -> Result<GetAttributesResponse, EnclaveError> {
        info!("Received get attributes request");
        let cluster_key = (self.cluster_handler.get_cluster_public_key().await).ok();
        let enclave_type = self.cluster_handler.get_cluster_type().await?;
        let response = GetAttributesResponse::builder()
            .tag(GIT_REVISION.to_string())
            .enclave_pcr0(self.context.settings.pcr0.clone())
            .enclave_type(enclave_type)
            .internal_public_key(self.context.settings.local_key.public_key().to_vec())
            .cluster_public_key(cluster_key)
            .build();
        Ok(response)
    }
}
