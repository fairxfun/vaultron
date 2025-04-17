use crate::error::EnclaveTesterError;
use anyhow::Result;
use enclave_agent::DEFAULT_ENCLAVE_AGENT_GRPC_SERVER_PORT;
use enclave_attestation::AttestationParser;
use enclave_grpc::connect;
use enclave_protos::vaultron::agent::v1::{
    enclave_agent_service_client::EnclaveAgentServiceClient, EnclaveAgentRequest, EnclaveAgentResponse,
};
use enclave_protos::vaultron::enclave::attestation::v1::AttestationDocument;
use enclave_protos::vaultron::enclave::v1::{EnclaveRequest, EnclaveResponse};
use enclave_protos::vaultron::service::v1::ClientOptions;
use log::error;
use prost::Message;
use serde::de::DeserializeOwned;
use std::sync::Arc;
use tonic::transport::Channel;
use typed_builder::TypedBuilder;

#[derive(TypedBuilder)]
pub struct MessageHandler {
    pub parser: Arc<AttestationParser>,
    pub client: EnclaveAgentServiceClient<Channel>,
    pub pcr0: Vec<u8>,
}

impl MessageHandler {
    pub async fn new(pcr0: Vec<u8>) -> Result<Self, EnclaveTesterError> {
        let parser = Arc::new(AttestationParser::new(None, None).await?);
        let client_options = ClientOptions::builder()
            .port(DEFAULT_ENCLAVE_AGENT_GRPC_SERVER_PORT)
            .build();
        let client = EnclaveAgentServiceClient::new(connect(&client_options).await.unwrap());
        Ok(Self::builder().parser(parser).client(client).pcr0(pcr0).build())
    }

    pub async fn send_enclave_agent_request(
        &mut self,
        request: EnclaveAgentRequest,
    ) -> Result<EnclaveAgentResponse, EnclaveTesterError> {
        let response = self.client.enclave_agent_request(tonic::Request::new(request)).await?;
        Ok(response.into_inner())
    }

    pub async fn send_enclave_request<R, P>(
        &mut self,
        request: EnclaveRequest,
    ) -> Result<(R, P, AttestationDocument), EnclaveTesterError>
    where
        R: DeserializeOwned + Message + Default,
        P: DeserializeOwned + Message + Default,
    {
        let response = self.client.enclave_request(tonic::Request::new(request)).await?;
        self.parse_enclave_response::<R, P>(response.get_ref())
    }

    fn parse_enclave_response<R, P>(
        &self,
        response: &EnclaveResponse,
    ) -> Result<(R, P, AttestationDocument), EnclaveTesterError>
    where
        R: DeserializeOwned + Message + Default,
        P: DeserializeOwned + Message + Default,
    {
        response.is_success().map_err(|err| {
            error!("Enclave proto error: {:?}", err);
            EnclaveTesterError::EnclaveResponseError
        })?;
        let document = self
            .parser
            .verify_and_parse(&response.attestation_document, &self.pcr0)?;

        let (r, p) = document.decode_user_data::<R, P>()?;
        Ok((r, p, document))
    }
}
