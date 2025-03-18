use super::MessageHandler;
use crate::common::EnclaveError;
use enclave_protos::enclave::v1::{
    InitKmstoolRequest, InitKmstoolResponse, UpdateAwsCredentialsRequest, UpdateAwsCredentialsResponse,
};
use log::info;

impl MessageHandler {
    pub(crate) async fn handle_kmstool_init_request(
        &self,
        request: InitKmstoolRequest,
    ) -> Result<InitKmstoolResponse, EnclaveError> {
        info!("Received init kmstool request: {:?}", request);

        self.context.kms_client.init(request.into())?;
        Ok(InitKmstoolResponse::success())
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
