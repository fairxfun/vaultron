use crate::EnclaveAgentCreateOptions;
use crate::{error::EnclaveAgentError, EnclaveAgentTrait};
use enclave_protos::enclave::v1::{
    InitRequest, InitResponse, UpdateAwsCredentialsRequest, UpdateAwsCredentialsResponse,
};
use enclave_vsock::{create_vsock_client, VsockClientCreateOptions, VsockClientTrait};
use prost::Message;
use std::sync::Arc;
use typed_builder::TypedBuilder;

type EnclaveVsockClient = Box<dyn VsockClientTrait>;

#[derive(Debug, TypedBuilder)]
pub(crate) struct EnclaveAgent {
    client: Arc<EnclaveVsockClient>,
}

impl EnclaveAgent {
    pub fn new(options: EnclaveAgentCreateOptions) -> Result<Self, EnclaveAgentError> {
        let vsock_client = create_vsock_client(
            VsockClientCreateOptions::builder()
                .cid(options.cid)
                .port(options.port)
                .build(),
        )?;
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
