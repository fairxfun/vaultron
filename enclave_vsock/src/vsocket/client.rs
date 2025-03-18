use crate::{VsockClientError, VsockClientTrait};
use std::{net::Shutdown, sync::Mutex};
use typed_builder::TypedBuilder;
use vsock::VsockStream;

use super::{create_vsock_protocol, VsockClientCreateOptions, VsockProtocol};

#[derive(Debug, TypedBuilder)]
pub struct VsockClient {
    pub server_cid: u32,
    pub server_port: u32,
    pub stream: Mutex<VsockStream>,
    pub protocol: VsockProtocol,
}

impl VsockClient {
    fn new(options: VsockClientCreateOptions) -> Result<Self, VsockClientError> {
        let stream = VsockStream::connect_with_cid_port(options.server_cid, options.server_port)?;
        let protocol = create_vsock_protocol();
        Ok(Self {
            server_cid: options.server_cid,
            server_port: options.server_port,
            stream: Mutex::new(stream),
            protocol,
        })
    }
}

#[async_trait::async_trait]
impl VsockClientTrait for VsockClient {
    async fn reconnect(&self) -> Result<(), VsockClientError> {
        let mut old_stream = self.stream.lock().map_err(|_| VsockClientError::StreamLockError)?;
        let _ = old_stream.shutdown(Shutdown::Both)?;
        let new_stream = VsockStream::connect_with_cid_port(self.server_cid, self.server_port)?;
        *old_stream = new_stream;
        Ok(())
    }

    async fn send_request(&self, message: &[u8]) -> Result<Vec<u8>, VsockClientError> {
        let mut stream = self.stream.lock().map_err(|_| VsockClientError::StreamLockError)?;
        self.protocol.write_message(&mut *stream, message)?;
        let response = self.protocol.read_message(&mut *stream)?;
        Ok(response)
    }
}

pub fn create_vsock_client(options: VsockClientCreateOptions) -> Result<VsockClient, VsockClientError> {
    VsockClient::new(options)
}
