use super::MessageHandler;
use crate::common::EnclaveError;
use enclave_protos::enclave::v1::{
    InitRequest, InitResponse, UpdateAwsCredentialsRequest, UpdateAwsCredentialsResponse,
};
use log::info;

impl MessageHandler {
    pub(crate) async fn handle_init_request(&self, request: InitRequest) -> Result<InitResponse, EnclaveError> {
        info!("Received init request: {:?}", request);

        self.context.kms_client.init(request.into())?;
        Ok(InitResponse::success())
    }

    pub(crate) async fn handle_update_aws_credentials_request(
        &self,
        request: UpdateAwsCredentialsRequest,
    ) -> Result<UpdateAwsCredentialsResponse, EnclaveError> {
        info!("Received update AWS credentials request: {:?}", request);
        self.context.kms_client.update_aws_credentials(request.into())?;
        Ok(UpdateAwsCredentialsResponse::success())
    }
}
