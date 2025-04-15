use crate::{EnclaveAgentError, EnclaveCreateOptions};
use anyhow::Result;
use enclave_protos::vaultron::enclave::v1::{EnclaveError as EnclaveProtoError, EnclaveRequest, EnclaveResponse};
use enclave_vsock::{create_vsock_client, VsockClientTrait};
use prost::Message;
use std::sync::Arc;

type EnclaveVsockClient = Box<dyn VsockClientTrait>;

pub struct EnclaveMessageHandler {
    client: Arc<EnclaveVsockClient>,
}

impl EnclaveMessageHandler {
    pub fn new(options: &EnclaveCreateOptions) -> Result<Self, EnclaveAgentError> {
        let vsock_client = create_vsock_client(options.into())?;
        //TODO: support reconnect
        let client = Arc::new(Box::new(vsock_client) as EnclaveVsockClient);
        Ok(Self { client })
    }

    pub async fn handle_request(&self, request: &EnclaveRequest) -> EnclaveResponse {
        let message = request.encode_to_vec();
        match self.client.send_request(message.as_slice()).await {
            Ok(response) => match EnclaveResponse::decode(&mut response.as_slice()) {
                Ok(response) => response,
                Err(_) => EnclaveProtoError::ResponseProtobufDecodeError.into(),
            },
            Err(_) => EnclaveProtoError::VsockClientError.into(),
        }
    }
}
