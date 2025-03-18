use super::{create_vsock_protocol, VsockProtocol, VsockServerCreateOptions};
use crate::{VsockMessageHandlerTrait, VsockServerError, VsockServerTrait};
use anyhow::Result;
use log::{error, info};
use std::fmt::Debug;
use std::sync::Arc;
use vsock::{get_local_cid, VsockListener, VsockStream};

pub type VsockMessageHandler<E> = Box<dyn VsockMessageHandlerTrait<Error = E>>;

#[derive(Debug)]
pub struct VsockServer<E: Debug + Send + Sync + 'static> {
    message_handler: Arc<VsockMessageHandler<E>>,
    protocol_handler: Arc<VsockProtocol>,
}

impl<E: Debug + Send + Sync + 'static> VsockServer<E> {
    fn new(message_handler: Arc<VsockMessageHandler<E>>) -> Self {
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
                Err(e) => {
                    error!("Error reading message: {:?}", e);
                    break;
                }
            };

            let response = match self.message_handler.process_message(&message).await {
                Ok(resp) => resp,
                Err(_) => {
                    error!("Error processing message");
                    break;
                }
            };

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
impl<E: Debug + Send + Sync + 'static> VsockServerTrait<E> for VsockServer<E> {
    async fn start(&self, options: VsockServerCreateOptions) -> Result<(), VsockServerError> {
        let cid = get_local_cid()?;
        info!("Starting vsock server with cid {} port {}", cid, options.port);
        let listener = VsockListener::bind_with_cid_port(cid, options.port)?;

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

pub fn create_vsock_server<E: Debug + Send + Sync + 'static>(
    message_handler: Arc<VsockMessageHandler<E>>,
) -> VsockServer<E> {
    VsockServer::new(message_handler)
}
