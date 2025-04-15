use enclave_agent::{EnclaveAgent, EnclaveAgentCreateOptions};
use enclave_attestation::AttestationParser;
// use enclave_protos::vaultron::enclave::attestation::v1::AttestationDocument;
// use enclave_protos::vaultron::enclave::v1::{EnclaveRequest, EnclaveResponse};
// use log::error;
// use prost::Message;
// use serde::de::DeserializeOwned;
use std::sync::Arc;
use typed_builder::TypedBuilder;

use crate::EnclaveTesterError;

#[derive(TypedBuilder)]
pub struct TesterMessageHandler {
    pub parser: Arc<AttestationParser>,
    pub agent: Arc<EnclaveAgent>,
    pub pcr0: Vec<u8>,
}

impl TesterMessageHandler {
    pub async fn new(pcr0: Vec<u8>, options: EnclaveAgentCreateOptions) -> Result<Self, EnclaveTesterError> {
        let parser = Arc::new(AttestationParser::new(None, None).await?);
        let agent = EnclaveAgent::new(&options).await?;
        Ok(Self::builder().parser(parser).agent(Arc::new(agent)).pcr0(pcr0).build())
    }

    // pub async fn send_request<R, P>(
    //     &self,
    //     request: &EnclaveRequest,
    // ) -> Result<(R, P, AttestationDocument), EnclaveTesterError>
    // where
    //     R: DeserializeOwned + Message + Default,
    //     P: DeserializeOwned + Message + Default,
    // {
    //     let response = self.agent.send_request_to_enclave(request).await?;
    //     self.parse_response::<R, P>(&response)
    // }

    // fn parse_response<R, P>(
    //     &self,
    //     response: &EnclaveResponse,
    // ) -> Result<(R, P, AttestationDocument), EnclaveTesterError>
    // where
    //     R: DeserializeOwned + Message + Default,
    //     P: DeserializeOwned + Message + Default,
    // {
    //     response.is_success().map_err(|err| {
    //         error!("Enclave proto error: {:?}", err);
    //         EnclaveTesterError::EnclaveResponseError
    //     })?;
    //     let document = self
    //         .parser
    //         .verify_and_parse(&response.attestation_document, &self.pcr0)?;

    //     let (r, p) = document.decode_user_data::<R, P>()?;
    //     Ok((r, p, document))
    // }
}
