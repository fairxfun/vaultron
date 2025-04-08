mod common;

use crate::common::MockTestService;
use bytes::{Buf, BufMut, Bytes, BytesMut};
use enclave_grpc::GrpcServer;
use enclave_protos::vaultron::service::v1::ServerOptions;
use enclave_protos::vaultron::test::v1::test_service_server::TestServiceServer;
use enclave_protos::vaultron::test::v1::{TestRequest, TestResponse};
use tonic::Response;

#[tokio::test]
async fn test_enable_web() {
    let options = ServerOptions::builder()
        .port(50151u32)
        .accept_http1(true)
        .enable_web(true)
        .build();
    let mut server = setup(options).await;
    let request = TestRequest::builder().message("hello world").build();
    let request = http::Request::builder()
        .version(http::Version::HTTP_11)
        .method(http::Method::POST)
        .uri("http://127.0.0.1:50151/vaultron.test.v1.TestService/Test")
        .header(http::header::CONTENT_TYPE, "application/grpc-web")
        .header(http::header::ACCEPT, "application/grpc-web")
        .body(hyper::Body::from(encode_body(request)))
        .unwrap();
    let client = hyper::Client::new();
    let response = client.request(request).await.unwrap();
    assert_eq!(
        response.headers().get(http::header::CONTENT_TYPE).unwrap(),
        "application/grpc-web+proto"
    );
    let body = response.into_body();
    let response = decode_body::<TestResponse>(body).await;
    assert_eq!(response.message, "hello world");
    server.stop().await.unwrap();
}
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

// one byte for the compression flag plus four bytes for the length
const GRPC_HEADER_SIZE: usize = 5;

fn encode_body<T>(msg: T) -> Bytes
where
    T: prost::Message,
{
    let msg_len = msg.encoded_len();
    let mut buf = BytesMut::with_capacity(GRPC_HEADER_SIZE + msg_len);

    // compression flag, 0 means "no compression"
    buf.put_u8(0);
    buf.put_u32(msg_len as u32);

    msg.encode(&mut buf).unwrap();
    buf.freeze()
}

async fn decode_body<T>(body: hyper::Body) -> T
where
    T: Default + prost::Message,
{
    let mut body = hyper::body::to_bytes(body).await.unwrap();

    // ignore the compression flag
    body.advance(1);

    let len = body.get_u32();
    #[allow(clippy::let_and_return)]
    let msg = T::decode(&mut body.split_to(len as usize)).unwrap();

    msg
}
