use crate::enclave_mock::MockKmstoolClient;
use enclave_kmstool::KmsToolTrait;
use enclave_protos::enclave::v1::{
    enclave_request, enclave_response, EnclaveRequest, EnclaveResponse, PingRequest, PingResponse, StatusCode,
};
use enclave_vsock::VsockMessageHandlerTrait;
use prost::Message;
use std::sync::Arc;
use vaultron_enclave::common::EnclaveConfig;
use vaultron_enclave::message::MessageHandler;
use vaultron_enclave::server::EnclaveServerContext;

#[tokio::test]
pub async fn test_enclave_ping_request() {
    let kmstool_client = MockKmstoolClient::new();
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
        .request(Some(enclave_request::Request::PingRequest(
            PingRequest::builder().build(),
        )))
        .build();
    let response = handler.process_message(&request.encode_to_vec()).await.unwrap();
    let response = EnclaveResponse::decode(&mut response.as_slice()).unwrap();
    let response: PingResponse = match response.response {
        Some(enclave_response::Response::PingResponse(response)) => response,
        _ => panic!("Invalid response"),
    };
    assert_eq!(response.code, Some(StatusCode::success()));
}
