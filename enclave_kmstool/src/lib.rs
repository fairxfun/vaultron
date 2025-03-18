mod error;
mod kmstool;

pub use error::*;
pub use kmstool::*;

#[cfg(feature = "kmstool_aws_clib_feature")]
use std::sync::Arc;
#[cfg(feature = "kmstool_aws_clib_feature")]
mod c_lib;

#[cfg(feature = "kmstool_aws_clib_feature")]
pub fn create_kmstool_clib_client() -> Arc<Box<dyn KmsToolTrait>> {
    Arc::new(Box::new(c_lib::KmsToolCLibClient::default()) as Box<dyn KmsToolTrait>)
}
