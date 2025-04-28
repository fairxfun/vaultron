use super::EnclaveAgent;
use crate::EnclaveAgentError;
use enclave_attestation::AttestationParser;
use enclave_protos::vaultron::enclave::internal::v1::{InitClusterKeyRequest, InitClusterKeyResponse};
use enclave_protos::vaultron::enclave::v1::{
    enclave_internal_request, enclave_request, EnclaveInternalRequest, EnclaveRequest, EnclaveResponse,
};
use prost::Message;
use serde::de::DeserializeOwned;
use service_discovery::VaultronServiceTags;
use std::sync::Arc;

impl EnclaveAgent {
    pub(crate) async fn try_init_enclave_cluster_startup(
        &self,
        enclave_debug_mode: bool,
    ) -> Result<VaultronServiceTags, EnclaveAgentError> {
        let pcr0 = if enclave_debug_mode {
            hex::decode(
                "000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000",
            )?
        } else {
            let enclave_info = self.agent_message_handler.describe_enclave().await?;
            let pcr0 = enclave_info
                .as_ref()
                .and_then(|info| info.measurements.as_ref())
                .map(|measurements| measurements.pcr0.clone())
                .ok_or(EnclaveAgentError::EnclaveInfoNotFoundError)?;
            hex::decode(pcr0)?
        };
        let parser = Arc::new(AttestationParser::new(None, None).await?);
        let response = self.send_init_enclave_cluster_key_request(&pcr0, &parser).await?;
        let cluster_public_key = response.cluster_public_key;
        Ok(VaultronServiceTags::builder()
            .pcr0(pcr0.to_vec())
            .cluster_public_key(cluster_public_key)
            .build())
    }

    async fn send_init_enclave_cluster_key_request(
        &self,
        pcr0: &[u8],
        parser: &Arc<AttestationParser>,
    ) -> Result<InitClusterKeyResponse, EnclaveAgentError> {
        let init_cluster_key_request = InitClusterKeyRequest::builder().build();
        let internal_request = EnclaveInternalRequest::builder()
            .request(enclave_internal_request::Request::InitClusterKeyRequest(
                init_cluster_key_request,
            ))
            .build();
        let request = EnclaveRequest::builder()
            .request(enclave_request::Request::InternalRequest(internal_request))
            .build();
        let response = self.enclave_message_handler.handle_request(&request).await;
        let (_, response) =
            parse_enclave_response::<InitClusterKeyRequest, InitClusterKeyResponse>(pcr0, parser, &response)?;
        Ok(response)
    }
}

fn parse_enclave_response<R, P>(
    pcr0: &[u8],
    parser: &Arc<AttestationParser>,
    response: &EnclaveResponse,
) -> Result<(R, P), EnclaveAgentError>
where
    R: DeserializeOwned + Message + Default,
    P: DeserializeOwned + Message + Default,
{
    response
        .is_success()
        .map_err(|e| EnclaveAgentError::EnclaveGrpcError(e.into()))?;
    let document = parser.verify_and_parse(&response.attestation_document, pcr0)?;
    let (r, p) = document.decode_user_data::<R, P>()?;
    Ok((r, p))
}
