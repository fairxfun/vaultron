mod common;

use crate::common::MockTestService;
use anyhow::Result;
use enclave_grpc::{connect, GrpcServer};
use enclave_protos::vaultron::service::v1::{ClientOptions, ServerOptions};
use enclave_protos::vaultron::test::v1::test_service_client::TestServiceClient;
use enclave_protos::vaultron::test::v1::test_service_server::TestServiceServer;
use enclave_protos::vaultron::test::v1::{TestRequest, TestResponse};
use rcgen::{generate_simple_self_signed, Certificate, CertificateParams};
use std::path::PathBuf;
use tonic::Response;

#[tokio::test]
async fn test_grpc_client() {
    let options = ServerOptions::builder().port(50051u32).build();
    let mut server = setup(options).await;
    let client_options = ClientOptions::builder().port(50051u32).build();
    let mut client = TestServiceClient::new(connect(&client_options).await.unwrap());
    let response = client
        .test(TestRequest::builder().message("hello world").build())
        .await
        .unwrap();
    assert_eq!(response.get_ref().message, "hello world");
    server.stop().await.unwrap();
}

#[tokio::test]
async fn test_grpc_client_with_ssl() {
    let tls_keys = TlsKeys::generate("example.com").unwrap();
    let options = ServerOptions::builder()
        .port(50052u32)
        .tls_key(tls_keys.server_key)
        .tls_pem(tls_keys.server_pem)
        .build();
    let mut server = setup(options).await;
    let client_options = ClientOptions::builder()
        .port(50052u32)
        .is_ssl(true)
        .ssl_cert(tls_keys.ca_pem)
        .ssl_server_name("example.com".to_string())
        .build();
    let mut client = TestServiceClient::new(connect(&client_options).await.unwrap());
    let response = client
        .test(TestRequest::builder().message("hello world").build())
        .await
        .unwrap();
    assert_eq!(response.get_ref().message, "hello world");
    server.stop().await.unwrap();
}

// #[tokio::test]
// async fn test_grpc_client_with_ssl_key_file() {
//     let tls_keys = TlsKeys::generate("127.0.0.1").unwrap();
//     let temp_folder = tempfile::tempdir().unwrap();
//     let ca_path = PathBuf::from(temp_folder.path()).join("ca.pem");
//     let server_pem_path = PathBuf::from(temp_folder.path()).join("server.pem");
//     let server_key_path = PathBuf::from(temp_folder.path()).join("server.key");

//     // Write all certificates to files
//     tokio::fs::write(ca_path.as_path(), tls_keys.ca_pem.clone())
//         .await
//         .unwrap();
//     tokio::fs::write(server_pem_path.as_path(), tls_keys.server_pem.clone())
//         .await
//         .unwrap();
//     tokio::fs::write(server_key_path.as_path(), tls_keys.server_key.clone())
//         .await
//         .unwrap();

//     let options = ServerOptions::builder()
//         .port(50053u32)
//         .tls_key_path(server_key_path.to_string_lossy().to_string())
//         .tls_pem_path(server_pem_path.to_string_lossy().to_string())
//         .build();
//     let mut server = setup(options).await;
//     let client_options = ClientOptions::builder()
//         .port(50053u32)
//         .is_ssl(true)
//         .ssl_cert_path(ca_path.to_string_lossy().to_string())
//         .ssl_server_name("127.0.0.1".to_string())
//         .build();
//     let mut client = TestServiceClient::new(connect(&client_options).await.unwrap());
//     let response = client
//         .test(TestRequest::builder().message("hello world").build())
//         .await
//         .unwrap();
//     assert_eq!(response.get_ref().message, "hello world");
//     server.stop().await.unwrap();
// }

async fn setup(options: ServerOptions) -> GrpcServer {
    let mut server = GrpcServer::default();
    let mut service = MockTestService::new();
    service.expect_test().returning(|request| {
        Ok(Response::new(
            TestResponse::builder()
                .message(request.get_ref().message.to_string())
                .build(),
        ))
    });
    server.start(TestServiceServer::new(service), options).await.unwrap();
    server
}

#[derive(Debug, Clone)]
pub struct TlsKeys {
    pub ca_pem: String,
    pub server_key: String,
    pub server_pem: String,
}

impl TlsKeys {
    pub fn generate(domain_name: &str) -> Result<Self> {
        let ca_cert = generate_simple_self_signed(vec![domain_name.to_string()])?;
        let ca_pem = ca_cert.serialize_pem()?;
        let mut server_params = CertificateParams::new(vec![domain_name.to_string()]);
        server_params.is_ca = rcgen::IsCa::NoCa;
        let server_cert = Certificate::from_params(server_params)?;
        let server_pem = server_cert.serialize_pem_with_signer(&ca_cert)?;
        let server_key = server_cert.serialize_private_key_pem();
        Ok(Self {
            ca_pem,
            server_key,
            server_pem,
        })
    }
}
