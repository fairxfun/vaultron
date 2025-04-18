use crate::{EnclaveAgentError, EnclaveCreateOptions};
use anyhow::Result;
use enclave_protos::vaultron::enclave::v1::{EnclaveError as EnclaveProtoError, EnclaveRequest, EnclaveResponse};
use enclave_vsock::{create_vsock_client, VsockClientTrait};
use log::{error, info};
use prost::Message;
use std::{sync::Arc, time::Duration};
use tokio::time::sleep;
type EnclaveVsockClient = Box<dyn VsockClientTrait>;

pub struct EnclaveMessageHandler {
    client: Arc<EnclaveVsockClient>,
}

impl EnclaveMessageHandler {
    pub async fn new(options: &EnclaveCreateOptions) -> Result<Self, EnclaveAgentError> {
        loop {
            let vsock_client = create_vsock_client(options.into());
            match vsock_client {
                Ok(client) => {
                    let client = Arc::new(Box::new(client) as EnclaveVsockClient);
                    return Ok(Self { client });
                }
                Err(e) => {
                    info!("Failed to create vsock client: {:?}, retry", e);
                    sleep(Duration::from_secs(5)).await;
                }
            }
        }
    }

    pub async fn handle_request(&self, request: &EnclaveRequest) -> EnclaveResponse {
        let message = request.encode_to_vec();
        match self.client.send_request(message.as_slice()).await {
            Ok(response) => match EnclaveResponse::decode(&mut response.as_slice()) {
                Ok(response) => response,
                Err(e) => {
                    error!("Failed to decode enclave response: {:?}", e);
                    EnclaveProtoError::ResponseProtobufDecodeError.into()
                }
            },
            Err(e) => {
                error!("Failed to send enclave request: {:?}", e);
                EnclaveProtoError::VsockClientError.into()
            }
        }
    }
}
