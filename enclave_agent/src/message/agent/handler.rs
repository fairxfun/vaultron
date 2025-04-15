use crate::enclave::EnclaveController;
use enclave_protos::vaultron::agent::v1::{
    enclave_agent_request, DescribeEnclaveResponse, EnclaveAgentRequest, EnclaveAgentResponse, RestartEnclaveResponse,
    StartEnclaveResponse, StopEnclaveResponse,
};
use enclave_protos::vaultron::common::v1::EnclaveAgentError as EnclaveAgentProtoError;
use std::sync::Arc;

pub struct EnclaveAgentMessageHandler {
    controller: Arc<EnclaveController>,
}

impl EnclaveAgentMessageHandler {
    pub fn new(controller: Arc<EnclaveController>) -> Self {
        Self { controller }
    }
}

impl EnclaveAgentMessageHandler {
    pub async fn handle_request(&self, request: &EnclaveAgentRequest) -> EnclaveAgentResponse {
        match &request.request {
            Some(enclave_agent_request::Request::StartRequest(_r)) => match self.controller.start_enclave().await {
                Ok(_) => StartEnclaveResponse::success().into(),
                Err(e) => EnclaveAgentResponse::error(e),
            },
            Some(enclave_agent_request::Request::StopRequest(_r)) => match self.controller.stop_enclave().await {
                Ok(_) => StopEnclaveResponse::success().into(),
                Err(e) => EnclaveAgentResponse::error(e),
            },
            Some(enclave_agent_request::Request::RestartRequest(_r)) => match self.controller.restart_enclave().await {
                Ok(_) => RestartEnclaveResponse::success().into(),
                Err(e) => EnclaveAgentResponse::error(e),
            },
            Some(enclave_agent_request::Request::DescribeRequest(_r)) => {
                match self.controller.describe_enclave().await {
                    Ok(enclave_info) => DescribeEnclaveResponse::success(enclave_info).into(),
                    Err(e) => EnclaveAgentResponse::error(e),
                }
            }
            None => EnclaveAgentResponse::error(EnclaveAgentProtoError::InvalidRequestError),
        }
    }
}
