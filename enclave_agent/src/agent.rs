use crate::error::EnclaveAgentError;
use crate::EnclaveAgentCreateOptions;
use enclave_vsock::{create_vsock_client, VsockClientTrait};
use prost::Message;
use std::sync::Arc;
use typed_builder::TypedBuilder;

type EnclaveVsockClient = Box<dyn VsockClientTrait>;

#[derive(TypedBuilder)]
pub struct EnclaveAgent {
    client: Arc<EnclaveVsockClient>,
}

impl EnclaveAgent {
    pub fn new(options: EnclaveAgentCreateOptions) -> Result<Self, EnclaveAgentError> {
        let vsock_client = create_vsock_client(options.into())?;
        let vsock_client = Arc::new(Box::new(vsock_client) as EnclaveVsockClient);
        Ok(EnclaveAgent::builder().client(vsock_client).build())
    }

    pub async fn send_request_to_enclave<R: Message, P: Message + Default>(
        &self,
        request: &R,
    ) -> Result<P, EnclaveAgentError> {
        let message = request.encode_to_vec();
        let response = self.client.send_request(message.as_slice()).await?;
        let response = P::decode(&mut response.as_slice())?;
        Ok(response)
    }
}
