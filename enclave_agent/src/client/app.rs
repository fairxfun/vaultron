use crate::EnclaveAgentCreateOptions;
use crate::{error::EnclaveAgentError, EnclaveAgentTrait};
use enclave_protos::enclave::v1::{
    InitRequest, InitResponse, UpdateAwsCredentialsRequest, UpdateAwsCredentialsResponse,
};
use enclave_vsock::{create_vsock_client, VsockClient, VsockClientCreateOptions};
use log::info;
use prost::Message;
use typed_builder::TypedBuilder;

#[derive(Debug, TypedBuilder)]
pub(crate) struct EnclaveAgent {
    client: VsockClient,
}

impl EnclaveAgent {
    pub fn new(options: EnclaveAgentCreateOptions) -> Result<Self, EnclaveAgentError> {
        let vsock_client = create_vsock_client(
            VsockClientCreateOptions::builder()
                .cid(options.cid)
                .port(options.port)
                .build(),
        )?;
        info!(
            "Connected to Enclave with cid: {} and port: {}",
            options.cid, options.port
        );
        Ok(EnclaveAgent::builder().client(vsock_client).build())
    }

    async fn send_request<T: Message, R: Message + Default>(&self, request: &T) -> Result<R, EnclaveAgentError> {
        self.client
            .send_request(request)
            .await
            .map_err(EnclaveAgentError::VsockClientError)
    }
}

#[async_trait::async_trait]
impl EnclaveAgentTrait for EnclaveAgent {
    async fn init(&self, request: InitRequest) -> Result<InitResponse, EnclaveAgentError> {
        self.send_request(&request).await
    }

    async fn update_aws_credentials(
        &self,
        request: UpdateAwsCredentialsRequest,
    ) -> Result<UpdateAwsCredentialsResponse, EnclaveAgentError> {
        self.send_request(&request).await
    }
}
