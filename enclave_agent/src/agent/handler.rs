use crate::{enclave_agent_trace_init, EnclaveAgentError, GIT_REVISION};
use enclave_grpc::GrpcServer;
use enclave_protos::vaultron::service::v1::ServerOptions;
use enclave_protos::vaultron::v1::enclave_agent_service_server::{EnclaveAgentService, EnclaveAgentServiceServer};
use enclave_protos::vaultron::v1::{EnclaveRequest, EnclaveResponse};
use enclave_vsock::{create_vsock_client, VsockClientTrait};
use enclave_vsock::{VsockClientCreateOptions, DEFAULT_ENCLAVE_CID, DEFAULT_ENCLAVE_VSOCK_PORT};
use log::info;
use prost::Message;
use std::sync::Arc;
use tonic::{Request, Response, Status};
use typed_builder::TypedBuilder;

const DEFAULT_ENCLAVE_AGENT_GRPC_SERVER_PORT: u32 = 8000;

type EnclaveVsockClient = Box<dyn VsockClientTrait>;

#[derive(Debug, Default, TypedBuilder)]
pub struct EnclaveAgentCreateOptions {
    #[builder(default = DEFAULT_ENCLAVE_CID)]
    pub enclave_cid: u32,
    #[builder(default = DEFAULT_ENCLAVE_VSOCK_PORT)]
    pub enclave_vsock_port: u32,
    #[builder(default = DEFAULT_ENCLAVE_AGENT_GRPC_SERVER_PORT)]
    pub enclave_agent_grpc_server_port: u32,
    #[builder(default = "info".to_string())]
    pub log_level: String,
}

impl From<&EnclaveAgentCreateOptions> for VsockClientCreateOptions {
    fn from(options: &EnclaveAgentCreateOptions) -> Self {
        VsockClientCreateOptions::builder()
            .enclave_cid(options.enclave_cid)
            .enclave_vsock_port(options.enclave_vsock_port)
            .build()
    }
}

#[derive(TypedBuilder)]
pub struct EnclaveAgent {
    client: Arc<EnclaveVsockClient>,
}

impl EnclaveAgent {
    pub fn new(options: &EnclaveAgentCreateOptions) -> Result<Self, EnclaveAgentError> {
        let vsock_client = create_vsock_client(options.into())?;
        //TODO: support reconnect
        let vsock_client = Arc::new(Box::new(vsock_client) as EnclaveVsockClient);
        Ok(EnclaveAgent::builder().client(vsock_client).build())
    }

    pub async fn send_request_to_enclave(
        &self,
        request: &EnclaveRequest,
    ) -> Result<EnclaveResponse, EnclaveAgentError> {
        let message = request.encode_to_vec();
        let response = self.client.send_request(message.as_slice()).await?;
        let response: EnclaveResponse = EnclaveResponse::decode(&mut response.as_slice())?;
        Ok(response)
    }
}

#[async_trait::async_trait]
impl EnclaveAgentService for EnclaveAgent {
    async fn request(&self, request: Request<EnclaveRequest>) -> Result<Response<EnclaveResponse>, Status> {
        let response = self.send_request_to_enclave(request.get_ref()).await;
        match response {
            Ok(response) => Ok(Response::new(response)),
            Err(e) => Err(Status::internal(e.to_string())),
        }
    }
}

pub async fn start_agent(options: EnclaveAgentCreateOptions) -> Result<(), EnclaveAgentError> {
    enclave_agent_trace_init(&options.log_level)?;
    info!("Start Enclave agent with git revision: {}", GIT_REVISION);
    let agent = EnclaveAgent::new(&options)?;
    let options = ServerOptions::builder()
        .port(options.enclave_agent_grpc_server_port)
        .accept_http1(false)
        .enable_web(false)
        .build();
    let mut server = GrpcServer::default();
    let result = server.start(EnclaveAgentServiceServer::new(agent), options).await;
    match result {
        Ok(_) => println!("Enclave agent exited without error"),
        Err(e) => println!("Enclave agent exited with error: {}", e),
    }
    Ok(())
}
