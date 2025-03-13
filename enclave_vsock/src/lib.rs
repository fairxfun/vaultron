mod error;
mod vsocket;

pub use error::*;
use std::fmt::Debug;
pub use vsocket::*;

#[async_trait::async_trait]
pub trait VsockMessageHandlerTrait: Send + Sync + Debug {
    type Error;
    async fn process_message(&self, message: &[u8]) -> Result<Vec<u8>, Self::Error>;
}
