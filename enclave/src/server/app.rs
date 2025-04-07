use crate::common::{enclave_trace_init, EnclaveError};
use crate::message::create_message_handler;
use crate::server::EnclaveServerContext;
use anyhow::{anyhow, Result};
use enclave_vsock::{create_vsock_server, VsockMessageHandlerTrait, VsockServerCreateOptions, VsockServerTrait};
use log::{error, info};
use std::sync::Arc;
use typed_builder::TypedBuilder;

#[derive(TypedBuilder)]
pub struct EnclaveServer {
    pub context: Arc<EnclaveServerContext>,
    pub vsock_server: Arc<Box<dyn VsockServerTrait>>,
}

impl EnclaveServer {
    pub fn new(context: Arc<EnclaveServerContext>) -> Result<Self, EnclaveError> {
        enclave_trace_init(&context.settings.log_level)?;
        let message_handler = create_message_handler(context.clone());
        let message_handler = Arc::new(Box::new(message_handler) as Box<dyn VsockMessageHandlerTrait>);
        let vsock_server = create_vsock_server(message_handler);
        let vsock_server = Arc::new(Box::new(vsock_server) as Box<dyn VsockServerTrait>);

        Ok(EnclaveServer::builder()
            .context(context)
            .vsock_server(vsock_server)
            .build())
    }

    pub async fn start(&self) -> Result<(), EnclaveError> {
        let port = self.context.settings.port;
        info!("Starting enclave server on port {}", port);
        let result = self
            .vsock_server
            .start(VsockServerCreateOptions::builder().enclave_vsock_port(port).build())
            .await;
        if let Err(e) = result {
            error!("Failed to start vsock server: {}", e);
            return Err(EnclaveError::AnyhowError(anyhow!("Failed to start vsock server")));
        }
        Ok(())
    }
}

pub async fn start_server() -> Result<(), EnclaveError> {
    let context = EnclaveServerContext::new().await?;
    let server = EnclaveServer::new(Arc::new(context))?;
    server.start().await
}
