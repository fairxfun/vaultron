mod error;
mod vsocket;

pub use error::*;
pub use vsocket::*;

use std::fmt::Debug;

#[async_trait::async_trait]
pub trait VsockMessageHandlerTrait: Send + Sync + Debug {
    type Error;
    async fn process_message(&self, message: &[u8]) -> Result<Vec<u8>, Self::Error>;
}

#[async_trait::async_trait]
pub trait VsockServerTrait<E: Debug + Send + Sync + 'static>: Send + Sync + Debug {
    async fn start(&self, options: VsockServerCreateOptions) -> Result<(), VsockServerError>;
}

#[async_trait::async_trait]
pub trait VsockClientTrait: Send + Sync + Debug {
    async fn reconnect(&self) -> Result<(), VsockClientError>;
    async fn send_request(&self, message: &[u8]) -> Result<Vec<u8>, VsockClientError>;
}
