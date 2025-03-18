mod client;
mod protocol;
mod server;

pub use client::*;
pub use protocol::*;
pub use server::*;
use typed_builder::TypedBuilder;

pub const DEFAULT_VSOCK_CID: u32 = 1000;
pub const DEFAULT_VSOCK_PORT: u32 = 5000;

#[derive(Debug, TypedBuilder)]
pub struct VsockClientCreateOptions {
    #[builder(default = DEFAULT_VSOCK_CID)]
    pub server_cid: u32,
    #[builder(default = DEFAULT_VSOCK_PORT)]
    pub server_port: u32,
}

#[derive(Debug, TypedBuilder)]
pub struct VsockServerCreateOptions {
    #[builder(default = DEFAULT_VSOCK_PORT)]
    pub port: u32,
}
