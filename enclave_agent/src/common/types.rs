use super::EnclaveAgentControllerError;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
pub enum EnclaveDescribeStatus {
    Unknown,
    Running,
    Terminating,
    Stopped,
    Empty,
}

impl FromStr for EnclaveDescribeStatus {
    type Err = EnclaveAgentControllerError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "RUNNING" => Ok(EnclaveDescribeStatus::Running),
            "TERMINATING" => Ok(EnclaveDescribeStatus::Terminating),
            "STOPPED" => Ok(EnclaveDescribeStatus::Stopped),
            "EMPTY" => Ok(EnclaveDescribeStatus::Empty),
            _ => Ok(EnclaveDescribeStatus::Unknown),
        }
    }
}
