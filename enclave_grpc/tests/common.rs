use anyhow::Result;
use enclave_protos::vaultron::test::v1::{TestRequest, TestResponse};
use mockall::mock;
use tonic::{Request, Response, Status};

mock! {
    pub TestService {}

    #[tonic::async_trait]
    impl enclave_protos::vaultron::test::v1::test_service_server::TestService for TestService {
        async fn test(&self, request: Request<TestRequest>) -> Result<Response<TestResponse>, Status>;
    }
}
