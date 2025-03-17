use crate::{VsockClientError, VsockClientTrait};
use std::sync::Mutex;
use typed_builder::TypedBuilder;
use vsock::VsockStream;

use super::{create_vsock_protocol, VsockProtocol};

#[derive(Debug, TypedBuilder)]
pub struct VsockClientCreateOptions {
    pub cid: u32,
    pub port: u32,
}

#[derive(Debug, TypedBuilder)]
pub struct VsockClient {
    pub cid: u32,
    pub port: u32,
    pub stream: Mutex<VsockStream>,
    pub protocol: VsockProtocol,
}

impl VsockClient {
    fn new(options: VsockClientCreateOptions) -> Result<Self, VsockClientError> {
        // TODO: support reconnect
        let stream = VsockStream::connect_with_cid_port(options.cid, options.port)?;
        let protocol = create_vsock_protocol();
        Ok(Self {
            cid: options.cid,
            port: options.port,
            stream: Mutex::new(stream),
            protocol,
        })
    }
}

#[async_trait::async_trait]
impl VsockClientTrait for VsockClient {
    async fn send_request(&self, message: &[u8]) -> Result<Vec<u8>, VsockClientError> {
        let mut stream = self.stream.lock().map_err(|_| VsockClientError::LockError)?;
        self.protocol.write_message(&mut *stream, message)?;
        let response = self.protocol.read_message(&mut *stream)?;
        Ok(response)
    }
}

pub fn create_vsock_client(options: VsockClientCreateOptions) -> Result<VsockClient, VsockClientError> {
    VsockClient::new(options)
}
