use crate::EnclaveAgentCreateOptions;
use crate::{error::EnclaveAgentError, EnclaveAgentTrait};
use enclave_protos::enclave::v1::{
    enclave_request, enclave_response, CreateEnclaveWalletRequest, CreateEnclaveWalletResponse, EnclaveRequest,
    EnclaveResponse, InitKmstoolRequest, InitKmstoolResponse, PingRequest, PingResponse, UpdateAwsCredentialsRequest,
    UpdateAwsCredentialsResponse,
};
use enclave_vsock::{create_vsock_client, VsockClientTrait};
use prost::Message;
use std::sync::Arc;
use typed_builder::TypedBuilder;

type EnclaveVsockClient = Box<dyn VsockClientTrait>;

#[derive(Debug, TypedBuilder)]
pub struct EnclaveAgent {
    client: Arc<EnclaveVsockClient>,
}

impl EnclaveAgent {
    pub fn new(options: EnclaveAgentCreateOptions) -> Result<Self, EnclaveAgentError> {
        let vsock_client = create_vsock_client(options.into())?;
        let vsock_client = Arc::new(Box::new(vsock_client) as EnclaveVsockClient);
        Ok(EnclaveAgent::builder().client(vsock_client).build())
    }

    async fn send_request<T: Message, R: Message + Default>(&self, request: &T) -> Result<R, EnclaveAgentError> {
        let message = request.encode_to_vec();
        let response = self.client.send_request(message.as_slice()).await?;
        let response = R::decode(&mut response.as_slice())?;
        Ok(response)
    }
}

#[async_trait::async_trait]
impl EnclaveAgentTrait for EnclaveAgent {
    async fn reconnect(&self) -> Result<(), EnclaveAgentError> {
        self.client
            .reconnect()
            .await
            .map_err(EnclaveAgentError::VsockClientError)
    }

    async fn ping(&self, request: PingRequest) -> Result<PingResponse, EnclaveAgentError> {
        let enclave_request = EnclaveRequest::builder()
            .request(enclave_request::Request::PingRequest(request))
            .build();
        let response: EnclaveResponse = self.send_request(&enclave_request).await?;
        match response.response {
            Some(enclave_response::Response::PingResponse(response)) => Ok(response),
            _ => Err(EnclaveAgentError::InvalidResponseError),
        }
    }

    async fn kmstool_init(&self, request: InitKmstoolRequest) -> Result<InitKmstoolResponse, EnclaveAgentError> {
        let enclave_request = EnclaveRequest::builder()
            .request(enclave_request::Request::InitKmstoolRequest(request))
            .build();
        let response: EnclaveResponse = self.send_request(&enclave_request).await?;
        match response.response {
            Some(enclave_response::Response::InitKmstoolResponse(response)) => Ok(response),
            _ => Err(EnclaveAgentError::InvalidResponseError),
        }
    }

    async fn update_aws_credentials(
        &self,
        request: UpdateAwsCredentialsRequest,
    ) -> Result<UpdateAwsCredentialsResponse, EnclaveAgentError> {
        let enclave_request = EnclaveRequest::builder()
            .request(enclave_request::Request::UpdateAwsCredentialsRequest(request))
            .build();
        let response: EnclaveResponse = self.send_request(&enclave_request).await?;
        match response.response {
            Some(enclave_response::Response::UpdateAwsCredentialsResponse(response)) => Ok(response),
            _ => Err(EnclaveAgentError::InvalidResponseError),
        }
    }

    async fn create_enclave_wallet(
        &self,
        request: CreateEnclaveWalletRequest,
    ) -> Result<CreateEnclaveWalletResponse, EnclaveAgentError> {
        let enclave_request = EnclaveRequest::builder()
            .request(enclave_request::Request::CreateEnclaveWalletRequest(request))
            .build();
        let response: EnclaveResponse = self.send_request(&enclave_request).await?;
        match response.response {
            Some(enclave_response::Response::CreateEnclaveWalletResponse(response)) => Ok(response),
            _ => Err(EnclaveAgentError::InvalidResponseError),
        }
    }
}
