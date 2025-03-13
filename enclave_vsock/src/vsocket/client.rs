use prost::Message;
use std::sync::Mutex;
use typed_builder::TypedBuilder;
use vsock::VsockStream;

use crate::EnclaveVsockClientError;

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
    fn new(options: VsockClientCreateOptions) -> Result<Self, EnclaveVsockClientError> {
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

    pub async fn send_request<T: Message, R: Message + Default>(
        &self,
        request: &T,
    ) -> Result<R, EnclaveVsockClientError> {
        let encoded_request = request.encode_to_vec();
        let mut stream = self.stream.lock().map_err(|_| EnclaveVsockClientError::LockError)?;
        self.protocol.write_message(&mut stream, &encoded_request)?;
        let response = self.protocol.read_message(&mut stream)?;
        Ok(R::decode(&response[..])?)
    }
}

pub fn create_vsock_client(options: VsockClientCreateOptions) -> Result<VsockClient, EnclaveVsockClientError> {
    VsockClient::new(options)
}
