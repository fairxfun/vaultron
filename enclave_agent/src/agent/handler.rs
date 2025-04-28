use crate::enclave::EnclaveController;
use crate::message::{EnclaveAgentMessageHandler, EnclaveMessageHandler};
use crate::{
    enclave_agent_trace_init, get_ec2_instance_id, get_ec2_instance_ip, EnclaveAgentError,
    EnclaveAgentServiceAttributes, ENCLAVE_AGENT_GIT_REVISION, ENCLAVE_AGENT_VERSION,
};
use enclave_grpc::GrpcServer;
use enclave_protos::vaultron::agent::v1::enclave_agent_service_server::{
    EnclaveAgentService, EnclaveAgentServiceServer,
};
use enclave_protos::vaultron::agent::v1::{EnclaveAgentRequest, EnclaveAgentResponse};
use enclave_protos::vaultron::enclave::v1::{EnclaveRequest, EnclaveResponse};
use enclave_protos::vaultron::service::v1::ServerOptions;
use enclave_vsock::VsockClientCreateOptions;
use log::{error, info, warn};
use serde::{Deserialize, Serialize};
use service_discovery::{create_service_discovery_register, VaultronServiceConfig, VaultronServiceRegisterTrait};
use std::sync::Arc;
use tokio::signal::unix::{signal, SignalKind};
use tonic::{Request, Response, Status};
use typed_builder::TypedBuilder;

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct CreateOptions {
    pub ec2_instance_options: Ec2InstanceOptions,
    pub agent_create_options: AgentCreateOptions,
    pub enclave_create_options: EnclaveCreateOptions,
}

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct Ec2InstanceOptions {
    pub instance_id: String,
    pub instance_address: String,
}

impl Ec2InstanceOptions {
    pub async fn new() -> Result<Self, EnclaveAgentError> {
        let instance_id = get_ec2_instance_id().await?;
        let instance_address = get_ec2_instance_ip().await?;
        Ok(Self::builder()
            .instance_id(instance_id)
            .instance_address(instance_address)
            .build())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct AgentCreateOptions {
    pub service_options: AgentServiceOptions,
    pub log_level: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct AgentServiceOptions {
    pub region: String,
    pub namespace: String,
    pub service_name: String,
    pub port: u32,
}

impl From<&AgentServiceOptions> for VaultronServiceConfig {
    fn from(options: &AgentServiceOptions) -> Self {
        VaultronServiceConfig::builder()
            .region(options.region.clone())
            .namespace(options.namespace.clone())
            .service_name(options.service_name.clone())
            .build()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct EnclaveCreateOptions {
    pub name: String,
    pub cid: u32,
    pub vsock_port: u32,
    pub cpu_count: u32,
    pub memory_size: u32,
    pub elf_file: String,
    pub debug_mode: bool,
    pub cluster_init_startup: bool,
}

impl From<&EnclaveCreateOptions> for VsockClientCreateOptions {
    fn from(options: &EnclaveCreateOptions) -> Self {
        VsockClientCreateOptions::builder()
            .enclave_cid(options.cid)
            .enclave_vsock_port(options.vsock_port)
            .build()
    }
}

#[derive(TypedBuilder)]
pub struct EnclaveAgent {
    pub(crate) enclave_message_handler: Arc<EnclaveMessageHandler>,
    pub(crate) agent_message_handler: Arc<EnclaveAgentMessageHandler>,
}

impl EnclaveAgent {
    async fn new(options: &CreateOptions) -> Result<Self, EnclaveAgentError> {
        let controller = Arc::new(EnclaveController::new(options.enclave_create_options.clone()));
        info!("try starting enclave");
        controller.try_start_enclave().await?;
        info!("enclave started successfully");
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

pub async fn start_enclave_agent(options: CreateOptions) -> Result<(), EnclaveAgentError> {
    enclave_agent_trace_init(&options.agent_create_options.log_level)?;
    info!(
        "Start Enclave agent with version: {} and git revision: {} options: {:?}",
        ENCLAVE_AGENT_VERSION, ENCLAVE_AGENT_GIT_REVISION, options
    );
    let service_register = create_enclave_agent_service_register(&options).await?;
    let agent = EnclaveAgent::new(&options).await?;
    let service_tags = service_register.get_service_tags().await?;
    if options.enclave_create_options.cluster_init_startup {
        info!("try initializing enclave cluster");
        if service_tags.is_some() {
            error!("Cluster already initialized");
            return Err(EnclaveAgentError::ClusterAlreadyInitializedError);
        }
        let service_tags = agent
            .try_init_enclave_cluster_startup(options.enclave_create_options.debug_mode)
            .await?;
        service_register.update_service_tags(service_tags).await?;
        info!("updated vaultron service tags successfully");
    } else if service_tags.is_none() {
        return Err(EnclaveAgentError::EnclaveServiceTagEmptyError);
    }

    let server_options = ServerOptions::builder()
        .port(options.agent_create_options.service_options.port)
        .accept_http1(false)
        .enable_web(false)
        .build();
    let mut server = GrpcServer::default();
    server
        .start(EnclaveAgentServiceServer::new(agent), server_options)
        .await?;
    info!("Enclave agent started");
    service_register.register_instance().await?;
    wait_for_signal().await?;
    let _ = server.stop().await;
    service_register.deregister_instance().await?;
    warn!("Enclave agent stopped");
    Ok(())
}

async fn create_enclave_agent_service_register(
    options: &CreateOptions,
) -> Result<Arc<Box<dyn VaultronServiceRegisterTrait<EnclaveAgentServiceAttributes>>>, EnclaveAgentError> {
    let attributes = EnclaveAgentServiceAttributes::builder()
        .host(options.ec2_instance_options.instance_address.clone())
        .port(options.agent_create_options.service_options.port)
        .build();
    let service_register = create_service_discovery_register::<EnclaveAgentServiceAttributes>(
        (&options.agent_create_options.service_options).into(),
        attributes,
        options.ec2_instance_options.instance_id.clone(),
    )
    .await?;
    Ok(service_register)
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
