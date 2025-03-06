use anyhow::Result;
use std::str::FromStr;

use super::EnclaveError;

pub fn enclave_trace_init(log_level: &str) -> Result<(), EnclaveError> {
    let _ = env_logger::builder()
        .filter_module("", log::LevelFilter::from_str(log_level)?)
        .try_init();
    Ok(())
}
