use crate::enclave::EnclaveController;
use crate::message::{EnclaveAgentMessageHandler, EnclaveMessageHandler};
use crate::{enclave_agent_trace_init, EnclaveAgentError, GIT_REVISION};
use enclave_grpc::GrpcServer;
use enclave_protos::vaultron::agent::v1::enclave_agent_service_server::{
    EnclaveAgentService, EnclaveAgentServiceServer,
};
use enclave_protos::vaultron::agent::v1::{EnclaveAgentRequest, EnclaveAgentResponse};
use enclave_protos::vaultron::enclave::v1::{EnclaveRequest, EnclaveResponse};
use enclave_protos::vaultron::service::v1::ServerOptions;
use enclave_vsock::{VsockClientCreateOptions, DEFAULT_ENCLAVE_CID, DEFAULT_ENCLAVE_VSOCK_PORT};
use log::info;
use std::sync::Arc;
use tonic::{Request, Response, Status};
use typed_builder::TypedBuilder;

pub const DEFAULT_ENCLAVE_AGENT_GRPC_SERVER_PORT: u32 = 8000;
pub const DEFAULT_ENCLAVE_CPU_COUNT: u32 = 2;
pub const DEFAULT_ENCLAVE_MEMORY_SIZE: u32 = 1024;
pub const DEFAULT_ENCLAVE_NAME: &str = "vaultron_enclave";
pub const DEFAULT_ENCLAVE_LOG_LEVEL: &str = "info";
pub const DEFAULT_ENCLAVE_ELF_FILE: &str = "/fairx/vaultron_enclave.eif";

#[derive(Debug, Default, TypedBuilder)]
#[builder(field_defaults(default, setter(into)))]
pub struct EnclaveAgentCreateOptions {
    #[builder(default = EnclaveCreateOptions::default())]
    pub enclave_create_options: EnclaveCreateOptions,
    #[builder(default = DEFAULT_ENCLAVE_AGENT_GRPC_SERVER_PORT)]
    pub enclave_agent_grpc_server_port: u32,
    #[builder(default = DEFAULT_ENCLAVE_LOG_LEVEL.to_string())]
    pub log_level: String,
}

#[derive(Debug, Clone, Default, TypedBuilder)]
#[builder(field_defaults(default, setter(into)))]
pub struct EnclaveCreateOptions {
    #[builder(default = DEFAULT_ENCLAVE_NAME.to_string())]
    pub enclave_name: String,
    #[builder(default = DEFAULT_ENCLAVE_CID)]
    pub enclave_cid: u32,
    #[builder(default = DEFAULT_ENCLAVE_VSOCK_PORT)]
    pub enclave_vsock_port: u32,
    #[builder(default = DEFAULT_ENCLAVE_CPU_COUNT)]
    pub enclave_cpu_count: u32,
    #[builder(default = DEFAULT_ENCLAVE_MEMORY_SIZE)]
    pub enclave_memory_size: u32,
    #[builder(default = DEFAULT_ENCLAVE_ELF_FILE.to_string())]
    pub enclave_elf_file: String,
}

impl From<&EnclaveCreateOptions> for VsockClientCreateOptions {
    fn from(options: &EnclaveCreateOptions) -> Self {
        VsockClientCreateOptions::builder()
            .enclave_cid(options.enclave_cid)
            .enclave_vsock_port(options.enclave_vsock_port)
            .build()
    }
}

#[derive(TypedBuilder)]
pub struct EnclaveAgent {
    enclave_message_handler: Arc<EnclaveMessageHandler>,
    agent_message_handler: Arc<EnclaveAgentMessageHandler>,
}

impl EnclaveAgent {
    pub async fn new(options: &EnclaveAgentCreateOptions) -> Result<Self, EnclaveAgentError> {
        let controller = Arc::new(EnclaveController::new(options.enclave_create_options.clone()));
        controller.check_enclave_status().await?;
        let enclave_message_handler = Arc::new(EnclaveMessageHandler::new(&options.enclave_create_options)?);
        let agent_message_handler = Arc::new(EnclaveAgentMessageHandler::new(controller));
        Ok(EnclaveAgent::builder()
            .enclave_message_handler(enclave_message_handler)
            .agent_message_handler(agent_message_handler)
            .build())
    }
}

#[async_trait::async_trait]
impl EnclaveAgentService for EnclaveAgent {
    async fn enclave_request(&self, request: Request<EnclaveRequest>) -> Result<Response<EnclaveResponse>, Status> {
        let response = self.enclave_message_handler.handle_request(request.get_ref()).await;
        Ok(Response::new(response))
    }

    async fn enclave_agent_request(
        &self,
        request: Request<EnclaveAgentRequest>,
    ) -> Result<Response<EnclaveAgentResponse>, Status> {
        let response = self.agent_message_handler.handle_request(request.get_ref()).await;
        Ok(Response::new(response))
    }
}

pub async fn start_agent(options: EnclaveAgentCreateOptions) -> Result<(), EnclaveAgentError> {
    enclave_agent_trace_init(&options.log_level)?;
    info!("Start Enclave agent with git revision: {}", GIT_REVISION);
    let agent = EnclaveAgent::new(&options).await?;
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
