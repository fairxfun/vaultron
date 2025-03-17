use crate::enclave_mock::MockKmstoolClient;
use enclave_kmstool::KmsUpdateAwsCredentialsRequest;
use enclave_kmstool::{KmsInitRequest, KmsToolTrait};
use enclave_protos::enclave::v1::{
    enclave_request, enclave_response, EnclaveRequest, EnclaveResponse, InitKmstoolRequest, InitKmstoolResponse,
    StatusCode, UpdateAwsCredentialsRequest, UpdateAwsCredentialsResponse,
};
use enclave_vsock::VsockMessageHandlerTrait;
use mockall::predicate;
use prost::Message;
use std::sync::Arc;
use vaultron_enclave::common::EnclaveConfig;
use vaultron_enclave::message::MessageHandler;
use vaultron_enclave::server::EnclaveServerContext;

#[tokio::test]
pub async fn test_enclave_send_init_request() {
    let request = InitKmstoolRequest::builder()
        .enable_logging(true)
        .proxy_port(2)
        .aws_region("us-east-1".to_string())
        .aws_access_key_id("1".to_string())
        .aws_secret_access_key("2".to_string())
        .aws_session_token("3".to_string())
        .kms_key_id("4".to_string())
        .kms_encryption_algorithm("5".to_string())
        .build();
    let kms_init_request = KmsInitRequest::builder()
        .enable_logging(true)
        .proxy_port(2)
        .aws_region("us-east-1".to_string())
        .aws_access_key_id("1".to_string())
        .aws_secret_access_key("2".to_string())
        .aws_session_token("3".to_string())
        .kms_key_id("4".to_string())
        .kms_encryption_algorithm("5".to_string())
        .build();
    let mut kmstool_client = MockKmstoolClient::new();
    kmstool_client
        .expect_init()
        .with(predicate::eq(kms_init_request))
        .returning(|_| Ok(()));
    let kmstool_client = Arc::new(Box::new(kmstool_client) as Box<dyn KmsToolTrait>);
    let config = Arc::new(EnclaveConfig::default());
    let context = Arc::new(
        EnclaveServerContext::builder()
            .config(config)
            .kms_client(kmstool_client)
            .build(),
    );
    let handler = MessageHandler::builder().context(context).build();
    let request = EnclaveRequest::builder()
        .request(Some(enclave_request::Request::InitKmstoolRequest(request)))
        .build();
    let response = handler.process_message(&request.encode_to_vec()).await.unwrap();
    let response = EnclaveResponse::decode(&mut response.as_slice()).unwrap();
    let response: InitKmstoolResponse = match response.response {
        Some(enclave_response::Response::InitKmstoolResponse(response)) => response,
        _ => panic!("Invalid response"),
    };
    assert_eq!(response.code, Some(StatusCode::success()));
}

#[tokio::test]
pub async fn test_enclave_send_update_aws_credentials_request() {
    let request = UpdateAwsCredentialsRequest::builder()
        .aws_access_key_id("1".to_string())
        .aws_secret_access_key("2".to_string())
        .aws_session_token("3".to_string())
        .build();
    let kms_update_aws_credentials_request = KmsUpdateAwsCredentialsRequest::builder()
        .aws_access_key_id("1".to_string())
        .aws_secret_access_key("2".to_string())
        .aws_session_token("3".to_string())
        .build();
    let mut kmstool_client = MockKmstoolClient::new();
    kmstool_client
        .expect_update_aws_credentials()
        .with(predicate::eq(kms_update_aws_credentials_request))
        .returning(|_| Ok(()));
    let kmstool_client = Arc::new(Box::new(kmstool_client) as Box<dyn KmsToolTrait>);
    let config = Arc::new(EnclaveConfig::default());
    let context = Arc::new(
        EnclaveServerContext::builder()
            .config(config)
            .kms_client(kmstool_client)
            .build(),
    );
    let handler = MessageHandler::builder().context(context).build();
    let request = EnclaveRequest::builder()
        .request(Some(enclave_request::Request::UpdateAwsCredentialsRequest(request)))
        .build();
    let response = handler.process_message(&request.encode_to_vec()).await.unwrap();
    let response = EnclaveResponse::decode(&mut response.as_slice()).unwrap();
    let response: UpdateAwsCredentialsResponse = match response.response {
        Some(enclave_response::Response::UpdateAwsCredentialsResponse(response)) => response,
        _ => panic!("Invalid response"),
    };
    assert_eq!(response.code, Some(StatusCode::success()));
}
