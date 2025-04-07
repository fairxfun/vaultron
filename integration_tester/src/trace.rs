use crate::EnclaveTesterError;
use anyhow::Result;
use std::str::FromStr;

pub fn integration_trace_init(log_level: &str) -> Result<(), EnclaveTesterError> {
    let _ = env_logger::builder()
        .filter_module("", log::LevelFilter::from_str(log_level)?)
        .try_init();
    Ok(())
}
