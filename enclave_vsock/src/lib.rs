mod error;
mod vsocket;

pub use error::*;
pub use vsocket::*;

#[async_trait::async_trait]
pub trait VsockMessageHandlerTrait: Send + Sync {
    async fn process_request(&self, message: &[u8]) -> Vec<u8>;
}

#[async_trait::async_trait]
pub trait VsockServerTrait: Send + Sync {
    async fn start(&self, options: VsockServerCreateOptions) -> Result<(), VsockServerError>;
}

#[async_trait::async_trait]
pub trait VsockClientTrait: Send + Sync {
    async fn reconnect(&self) -> Result<(), VsockClientError>;
    async fn send_request(&self, message: &[u8]) -> Result<Vec<u8>, VsockClientError>;
}
