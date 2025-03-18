use crate::common::{enclave_trace_init, EnclaveError};
use crate::message::create_message_handler;
use crate::server::EnclaveServerContext;
use anyhow::{anyhow, Result};
use enclave_vsock::{create_vsock_server, VsockMessageHandlerTrait, VsockServerCreateOptions, VsockServerTrait};
use log::{error, info};
use std::sync::Arc;
use typed_builder::TypedBuilder;

type EnclaveVsockServer = Box<dyn VsockServerTrait<EnclaveError>>;

#[derive(Debug, TypedBuilder)]
pub struct EnclaveServer {
    pub context: Arc<EnclaveServerContext>,
    pub vsock_server: Arc<EnclaveVsockServer>,
}

impl EnclaveServer {
    pub fn new(context: Arc<EnclaveServerContext>) -> Result<Self, EnclaveError> {
        enclave_trace_init(&context.config.log_level)?;
        let message_handler = create_message_handler(context.clone());
        let message_handler =
            Arc::new(Box::new(message_handler) as Box<dyn VsockMessageHandlerTrait<Error = EnclaveError>>);
        let vsock_server = create_vsock_server::<EnclaveError>(message_handler);
        let vsock_server = Arc::new(Box::new(vsock_server) as EnclaveVsockServer);

        Ok(EnclaveServer::builder()
            .context(context)
            .vsock_server(vsock_server)
            .build())
    }

    pub async fn start(&self) -> Result<(), EnclaveError> {
        let port = self.context.config.port;
        info!("Starting enclave server on port {}", port);
        let result = self
            .vsock_server
            .start(VsockServerCreateOptions::builder().port(port).build())
            .await;
        if let Err(e) = result {
            error!("Failed to start vsock server: {}", e);
            return Err(EnclaveError::AnyhowError(anyhow!("Failed to start vsock server")));
        }
        Ok(())
    }
}

pub async fn start_server() -> Result<(), EnclaveError> {
    let context = EnclaveServerContext::new()?;
    let server = EnclaveServer::new(Arc::new(context))?;
    server.start().await
}
