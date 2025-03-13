use crate::EnclaveAgentError;
use enclave_protos::enclave::v1::{
    InitRequest, InitResponse, UpdateAwsCredentialsRequest, UpdateAwsCredentialsResponse,
};
use std::fmt::Debug;
use typed_builder::TypedBuilder;

#[derive(Debug, TypedBuilder)]
pub struct EnclaveAgentCreateOptions {
    pub cid: u32,
    pub port: u32,
}

#[async_trait::async_trait]
pub trait EnclaveAgentTrait: Send + Sync + Debug {
    async fn init(&self, request: InitRequest) -> Result<InitResponse, EnclaveAgentError>;
    async fn update_aws_credentials(
        &self,
        request: UpdateAwsCredentialsRequest,
    ) -> Result<UpdateAwsCredentialsResponse, EnclaveAgentError>;
}

#[async_trait::async_trait]
impl EnclaveAgentTrait for Box<dyn EnclaveAgentTrait> {
    async fn init(&self, request: InitRequest) -> Result<InitResponse, EnclaveAgentError> {
        self.as_ref().init(request).await
    }

    async fn update_aws_credentials(
        &self,
        request: UpdateAwsCredentialsRequest,
    ) -> Result<UpdateAwsCredentialsResponse, EnclaveAgentError> {
        self.as_ref().update_aws_credentials(request).await
    }
}
