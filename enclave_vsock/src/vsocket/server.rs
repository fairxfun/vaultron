use super::{create_vsock_protocol, VsockProtocol, VsockServerCreateOptions};
use crate::{VsockMessageHandlerTrait, VsockProtocolError, VsockServerError, VsockServerTrait};
use anyhow::Result;
use log::{error, info};
use std::sync::Arc;
use vsock::{get_local_cid, VsockListener, VsockStream};

pub struct VsockServer {
    message_handler: Arc<Box<dyn VsockMessageHandlerTrait>>,
    protocol_handler: Arc<VsockProtocol>,
}

impl VsockServer {
    fn new(message_handler: Arc<Box<dyn VsockMessageHandlerTrait>>) -> Self {
        let protocol_handler = Arc::new(create_vsock_protocol());
        Self {
            message_handler,
            protocol_handler,
        }
    }

    async fn handle_client(&self, mut stream: VsockStream) -> Result<(), VsockServerError> {
        loop {
            let message = match self.protocol_handler.read_message(&mut stream) {
                Ok(msg) => msg,
                Err(e) => match e {
                    VsockProtocolError::IoError(io_err) => {
                        if io_err.kind() == std::io::ErrorKind::UnexpectedEof {
                            info!("Client disconnected gracefully");
                            break;
                        } else {
                            error!("Error reading message: {:?}", io_err);
                            break;
                        }
                    }
                    _ => {
                        error!("Error reading message: {:?}", e);
                        break;
                    }
                },
            };

            let response = self.message_handler.process_request(&message).await;

            if let Err(e) = self.protocol_handler.write_message(&mut stream, &response) {
                error!("Error writing response: {:?}", e);
                break;
            }
        }

        info!("Client disconnected!");
        Ok(())
    }
}

#[async_trait::async_trait]
impl VsockServerTrait for VsockServer {
    async fn start(&self, options: VsockServerCreateOptions) -> Result<(), VsockServerError> {
        let cid = get_local_cid()?;
        info!(
            "Starting vsock server with cid {} port {}",
            cid, options.enclave_vsock_port
        );
        let listener = VsockListener::bind_with_cid_port(cid, options.enclave_vsock_port)?;

        loop {
            match listener.accept() {
                Ok((stream, _)) => {
                    info!("Client connected!");
                    if let Err(e) = self.handle_client(stream).await {
                        error!("Error handling client: {:?}", e);
                    }
                }
                Err(e) => {
                    error!("Error accepting connection: {:?}", e);
                }
            }
        }
    }
}

pub fn create_vsock_server(message_handler: Arc<Box<dyn VsockMessageHandlerTrait>>) -> VsockServer {
    VsockServer::new(message_handler)
}
