use crate::{VsockClientError, VsockClientTrait};
use std::{net::Shutdown, sync::Mutex};
use typed_builder::TypedBuilder;
use vsock::VsockStream;

use super::{create_vsock_protocol, VsockClientCreateOptions, VsockProtocol};

#[derive(Debug, TypedBuilder)]
pub struct VsockClient {
    pub enclave_cid: u32,
    pub enclave_vsock_port: u32,
    pub stream: Mutex<VsockStream>,
    pub protocol: VsockProtocol,
}

impl VsockClient {
    fn new(options: VsockClientCreateOptions) -> Result<Self, VsockClientError> {
        let stream = VsockStream::connect_with_cid_port(options.enclave_cid, options.enclave_vsock_port)?;
        let protocol = create_vsock_protocol();
        Ok(Self {
            enclave_cid: options.enclave_cid,
            enclave_vsock_port: options.enclave_vsock_port,
            stream: Mutex::new(stream),
            protocol,
        })
    }
}

impl Drop for VsockClient {
    fn drop(&mut self) {
        let stream = self.stream.lock();
        if let Ok(stream) = stream {
            let _ = stream.shutdown(Shutdown::Both);
        }
    }
}

#[async_trait::async_trait]
impl VsockClientTrait for VsockClient {
    async fn reconnect(&self) -> Result<(), VsockClientError> {
        let mut old_stream = self.stream.lock().map_err(|_| VsockClientError::StreamLockError)?;
        let _ = old_stream.shutdown(Shutdown::Both)?;
        let new_stream = VsockStream::connect_with_cid_port(self.enclave_cid, self.enclave_vsock_port)?;
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
