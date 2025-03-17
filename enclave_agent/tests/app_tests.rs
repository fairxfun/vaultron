use std::sync::Arc;

use enclave_agent::{create_enclave_agent, EnclaveAgent, EnclaveAgentCreateOptions, EnclaveAgentTrait};
use enclave_protos::enclave::v1::SignatureType;
use enclave_protos::enclave::v1::{
    enclave_request, enclave_response, CreateEnclaveWalletRequest, CreateEnclaveWalletResponse, EnclaveRequest,
    EnclaveResponse, InitKmstoolRequest, InitKmstoolResponse, StatusCode, UpdateAwsCredentialsRequest,
    UpdateAwsCredentialsResponse,
};
use enclave_vsock::{VsockClientError, VsockClientTrait};
use mockall::mock;
use mockall::predicate;
use prost::Message;

mock! {
    #[derive(Debug)]
    pub VsockClient {}

    #[async_trait::async_trait]
    impl VsockClientTrait for VsockClient {
        async fn send_request(&self, message: &[u8]) -> Result<Vec<u8>, VsockClientError>;
    }
}

#[tokio::test]
pub async fn test_enclave_agent_send_init_request() {
    let mut client = MockVsockClient::new();
    let request: InitKmstoolRequest = InitKmstoolRequest::builder()
        .enable_logging(true)
        .proxy_port(2)
        .aws_region("us-east-1".to_string())
        .aws_access_key_id("1".to_string())
        .aws_secret_access_key("2".to_string())
        .aws_session_token("3".to_string())
        .kms_key_id("4".to_string())
        .kms_encryption_algorithm("5".to_string())
        .build();
    let encoded_request = EnclaveRequest::builder()
        .request(enclave_request::Request::InitKmstoolRequest(request.clone()))
        .build();
    client
        .expect_send_request()
        .with(predicate::eq(encoded_request.encode_to_vec()))
        .returning(|_| {
            let response = InitKmstoolResponse::builder().code(StatusCode::success()).build();
            let response = EnclaveResponse::builder()
                .response(enclave_response::Response::InitKmstoolResponse(response))
                .build();
            let buffer = response.encode_to_vec();
            Ok(buffer)
        });
    let client = Arc::new(Box::new(client) as Box<dyn VsockClientTrait>);
    let agent = EnclaveAgent::builder().client(client).build();
    let response = agent.kmstool_init(request).await.unwrap();
    assert!(response.code.unwrap().success);
}

#[tokio::test]
pub async fn test_enclave_agent_send_update_credentials_request() {
    let request: UpdateAwsCredentialsRequest = UpdateAwsCredentialsRequest::builder()
        .aws_access_key_id("1".to_string())
        .aws_secret_access_key("2".to_string())
        .aws_session_token("3".to_string())
        .build();
    let enclave_request = EnclaveRequest::builder()
        .request(enclave_request::Request::UpdateAwsCredentialsRequest(request.clone()))
        .build();
    let mut client = MockVsockClient::new();
    client
        .expect_send_request()
        .with(predicate::eq(enclave_request.encode_to_vec()))
        .returning(|_| {
            let response = UpdateAwsCredentialsResponse::builder()
                .code(StatusCode::success())
                .build();
            let response = EnclaveResponse::builder()
                .response(enclave_response::Response::UpdateAwsCredentialsResponse(response))
                .build();
            let buffer = response.encode_to_vec();
            Ok(buffer)
        });
    let client = Arc::new(Box::new(client) as Box<dyn VsockClientTrait>);
    let agent = EnclaveAgent::builder().client(client).build();
    let response = agent.update_aws_credentials(request).await.unwrap();
    assert!(response.code.unwrap().success);
}

#[tokio::test]
pub async fn test_create_enclave_wallet_request() {
    let request = CreateEnclaveWalletRequest::builder()
        .user_id(vec![1])
        .user_public_key(vec![2])
        .signature_type(SignatureType::WalletEth)
        .message("create enclave wallet".to_string())
        .signature(vec![3])
        .build();
    let encoded_request = EnclaveRequest::builder()
        .request(enclave_request::Request::CreateEnclaveWalletRequest(request.clone()))
        .build();
    let mut client = MockVsockClient::new();
    client
        .expect_send_request()
        .with(predicate::eq(encoded_request.encode_to_vec()))
        .returning(|_| {
            let response = CreateEnclaveWalletResponse::builder()
                .code(StatusCode::success())
                .build();
            let response = EnclaveResponse::builder()
                .response(enclave_response::Response::CreateEnclaveWalletResponse(response))
                .build();
            let buffer = response.encode_to_vec();
            Ok(buffer)
        });
    let client = Arc::new(Box::new(client) as Box<dyn VsockClientTrait>);
    let agent = EnclaveAgent::builder().client(client).build();
    let response = agent.create_enclave_wallet(request).await.unwrap();
    assert!(response.code.unwrap().success);
}

#[tokio::test]
pub async fn test_create_enclave_agent() {
    let agent = create_enclave_agent(EnclaveAgentCreateOptions::builder().cid(1).port(1).build());
    assert!(agent.is_err());
}
