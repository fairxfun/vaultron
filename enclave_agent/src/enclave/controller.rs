use log::{error, info};
use std::time::Duration;
use tokio::process::Command;
use tokio::time::sleep;
use typed_builder::TypedBuilder;

use crate::{EnclaveAgentControllerError, EnclaveCreateOptions, EnclaveDescribeStatus};

#[derive(Debug, TypedBuilder)]
pub struct EnclaveController {
    pub(crate) options: EnclaveCreateOptions,
}

impl EnclaveController {
    pub fn new(options: EnclaveCreateOptions) -> Self {
        Self { options }
    }

    pub async fn start_enclave(&self) -> Result<(), EnclaveAgentControllerError> {
        let status = self.get_enclave_status().await?;
        if status != EnclaveDescribeStatus::Empty {
            info!("enclave is already running");
            return Ok(());
        }

        self.do_start_enclave().await
    }

    pub async fn stop_enclave(&self) -> Result<(), EnclaveAgentControllerError> {
        let status = self.get_enclave_status().await?;
        if status == EnclaveDescribeStatus::Empty {
            info!("enclave is not running");
            return Ok(());
        }

        self.do_stop_enclave().await
    }

    pub async fn restart_enclave(&self) -> Result<(), EnclaveAgentControllerError> {
        let status = self.get_enclave_status().await?;
        if status == EnclaveDescribeStatus::Empty {
            self.start_enclave().await?;
        } else {
            self.stop_enclave().await?;
            sleep(Duration::from_secs(5)).await;
            self.start_enclave().await?;
        }

        Ok(())
    }

    pub async fn try_start_enclave(&self) -> Result<(), EnclaveAgentControllerError> {
        let status = self.get_enclave_status().await?;
        if status == EnclaveDescribeStatus::Running {
            info!("enclave is already running");
            return Ok(());
        }

        if status == EnclaveDescribeStatus::Empty {
            self.do_start_enclave().await?;
        } else {
            self.do_stop_enclave().await?;
            sleep(Duration::from_secs(5)).await;
            self.do_start_enclave().await?;
        }

        while self.get_enclave_status().await? != EnclaveDescribeStatus::Running {
            info!("waiting for enclave to start");
            sleep(Duration::from_secs(5)).await;
        }
        Ok(())
    }

    async fn do_start_enclave(&self) -> Result<(), EnclaveAgentControllerError> {
        let mut args = vec![
            "run-enclave".to_string(),
            "--enclave-name".to_string(),
            self.options.enclave_name.clone(),
            "--enclave-cid".to_string(),
            self.options.enclave_cid.to_string(),
            "--eif-path".to_string(),
            self.options.enclave_elf_file.to_string(),
            "--cpu-count".to_string(),
            self.options.enclave_cpu_count.to_string(),
            "--memory".to_string(),
            self.options.enclave_memory_size.to_string(),
        ];
        if self.options.debug_mode {
            args.push("--debug-mode".to_string());
        }
        let output = Command::new("nitro-cli").args(args).output().await?;
        if output.status.success() {
            Ok(())
        } else {
            error!("start enclave response: {:?}", output);
            Err(EnclaveAgentControllerError::StartEnclaveError(
                String::from_utf8_lossy(&output.stderr).to_string(),
            ))
        }
    }

    async fn do_stop_enclave(&self) -> Result<(), EnclaveAgentControllerError> {
        let output = Command::new("nitro-cli")
            .args(["terminate-enclave", "--enclave-name", &self.options.enclave_name])
            .output()
            .await?;
        if output.status.success() {
            Ok(())
        } else {
            error!("stop enclave response: {:?}", output);
            Err(EnclaveAgentControllerError::StopEnclaveError(
                String::from_utf8_lossy(&output.stderr).to_string(),
            ))
        }
    }
}
