use std::str::FromStr;

use super::EnclaveController;
use crate::{EnclaveAgentControllerError, EnclaveDescribeStatus};
use anyhow::Result;
use enclave_protos::vaultron::agent::v1::DescribeEnclaveInfo;
use tokio::process::Command;

impl EnclaveController {
    pub async fn get_enclave_status(&self) -> Result<EnclaveDescribeStatus, EnclaveAgentControllerError> {
        let enclave_info = self.describe_enclave().await?;
        if let Some(enclave_info) = enclave_info {
            EnclaveDescribeStatus::from_str(&enclave_info.state)
        } else {
            Ok(EnclaveDescribeStatus::Empty)
        }
    }

    pub async fn describe_enclave(&self) -> Result<Option<DescribeEnclaveInfo>, EnclaveAgentControllerError> {
        let output = Command::new("nitro-cli")
            .args(["describe-enclaves"])
            .output()
            .await
            .map_err(|e| EnclaveAgentControllerError::DescribeEnclaveError(e.to_string()))?;

        if output.status.success() {
            let infos: Vec<DescribeEnclaveInfo> = serde_json::from_str(&String::from_utf8_lossy(&output.stdout))?;
            for info in infos {
                if info.enclave_name == self.options.name {
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
