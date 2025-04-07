mod client;
mod protocol;
mod server;

pub use client::*;
pub use protocol::*;
pub use server::*;

use typed_builder::TypedBuilder;

pub const DEFAULT_ENCLAVE_CID: u32 = 1000;
pub const DEFAULT_ENCLAVE_VSOCK_PORT: u32 = 5000;

#[derive(Debug, TypedBuilder)]
pub struct VsockClientCreateOptions {
    #[builder(default = DEFAULT_ENCLAVE_CID)]
    pub enclave_cid: u32,
    #[builder(default = DEFAULT_ENCLAVE_VSOCK_PORT)]
    pub enclave_vsock_port: u32,
}

#[derive(Debug, TypedBuilder)]
pub struct VsockServerCreateOptions {
    #[builder(default = DEFAULT_ENCLAVE_VSOCK_PORT)]
    pub enclave_vsock_port: u32,
}
