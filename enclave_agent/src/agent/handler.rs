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
use enclave_vsock::VsockClientCreateOptions;
use log::info;
use std::sync::Arc;
use tokio::signal::unix::{signal, SignalKind};
use tonic::{Request, Response, Status};
use typed_builder::TypedBuilder;

#[derive(Debug, TypedBuilder)]
pub struct EnclaveAgentCreateOptions {
    pub enclave_create_options: EnclaveCreateOptions,
    pub enclave_agent_grpc_server_port: u32,
    pub log_level: String,
}

#[derive(Debug, Clone, TypedBuilder)]
pub struct EnclaveCreateOptions {
    pub enclave_name: String,
    pub enclave_cid: u32,
    pub enclave_vsock_port: u32,
    pub enclave_cpu_count: u32,
    pub enclave_memory_size: u32,
    pub enclave_elf_file: String,
    pub debug_mode: bool,
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
    async fn new(options: &EnclaveAgentCreateOptions) -> Result<Self, EnclaveAgentError> {
        let controller = Arc::new(EnclaveController::new(options.enclave_create_options.clone()));
        controller.try_start_enclave().await?;
        let enclave_message_handler = Arc::new(EnclaveMessageHandler::new(&options.enclave_create_options).await?);
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

pub async fn start_enclave_agent(options: EnclaveAgentCreateOptions) -> Result<(), EnclaveAgentError> {
    enclave_agent_trace_init(&options.log_level)?;
    info!("Start Enclave agent with git revision: {}", GIT_REVISION);
    let agent = EnclaveAgent::new(&options).await?;
    let options = ServerOptions::builder()
        .port(options.enclave_agent_grpc_server_port)
        .accept_http1(false)
        .enable_web(false)
        .build();
    let mut server = GrpcServer::default();
    server.start(EnclaveAgentServiceServer::new(agent), options).await?;
    info!("Enclave agent started");
    wait_for_signal().await?;
    let _ = server.stop().await;
    info!("Enclave agent stopped");
    Ok(())
}

async fn wait_for_signal() -> Result<(), EnclaveAgentError> {
    let mut sigint = signal(SignalKind::interrupt())?; // SIGINT
    let mut sigterm = signal(SignalKind::terminate())?; // SIGTERM
    tokio::select! {
        _ = sigint.recv() => info!("Received SIGINT (Ctrl+C)"),
        _ = sigterm.recv() => info!("Received SIGTERM (kill)"),
    }
    Ok(())
}
