use super::EnclaveController;
use crate::EnclaveAgentControllerError;
use anyhow::Result;
use enclave_protos::vaultron::agent::v1::DescribeEnclaveInfo;
use log::info;
use std::str::FromStr;
use tokio::process::Command;

#[derive(Debug, PartialEq, Eq)]
pub enum EnclaveStatus {
    Unknown,
    Running,
    Terminating,
    Stopped,
}

impl FromStr for EnclaveStatus {
    type Err = EnclaveAgentControllerError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Running" => Ok(EnclaveStatus::Running),
            "Terminating" => Ok(EnclaveStatus::Terminating),
            "Stopped" => Ok(EnclaveStatus::Stopped),
            _ => Ok(EnclaveStatus::Unknown),
        }
    }
}

impl EnclaveController {
    pub async fn check_enclave_status(&self) -> Result<(), EnclaveAgentControllerError> {
        let enclave_info = self.describe_enclave().await?;
        if let Some(enclave_info) = enclave_info {
            let status = EnclaveStatus::from_str(&enclave_info.state)?;
            if status == EnclaveStatus::Running {
                info!("Enclave is already running");
                return Ok(());
            }
        }
        self.restart_enclave().await?;
        Ok(())
    }

    pub async fn describe_enclave(&self) -> Result<Option<DescribeEnclaveInfo>, EnclaveAgentControllerError> {
        let output = Command::new("nitro-cli")
            .args(["describe-enclaves"])
            .output()
            .await
            .map_err(|e| EnclaveAgentControllerError::DescribeEnclaveError(e.to_string()))?;

        if output.status.success() {
            let infos: Vec<DescribeEnclaveInfo> = serde_json::from_slice(&output.stdout)?;
            for info in infos {
                if info.enclave_name == self.options.enclave_name {
                    return Ok(Some(info));
                }
            }
            Ok(None)
        } else {
            Err(EnclaveAgentControllerError::DescribeEnclaveError(
                String::from_utf8_lossy(&output.stderr).to_string(),
            ))
        }
    }
}
