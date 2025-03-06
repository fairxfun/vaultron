#[link(name = "kmstool-enclave-lib")]
unsafe extern "C" {}

mod c_lib;
mod error;
mod kmstool;

pub use c_lib::*;
pub use error::*;
pub use kmstool::*;
