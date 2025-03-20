mod error;
mod kmstool;

#[cfg(feature = "kmstool_aws_clib_feature")]
mod client;
#[cfg(feature = "kmstool_aws_clib_feature")]
mod gen;

pub use error::*;
pub use kmstool::*;

#[cfg(feature = "kmstool_aws_clib_feature")]
use std::sync::Arc;

#[cfg(feature = "kmstool_aws_clib_feature")]
pub fn create_kmstool_clib_client() -> Arc<Box<dyn KmsToolTrait>> {
    Arc::new(Box::new(client::KmsToolCLibClient::default()) as Box<dyn KmsToolTrait>)
}
