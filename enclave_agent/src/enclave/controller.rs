use log::warn;
use tokio::process::Command;
use typed_builder::TypedBuilder;

use crate::{EnclaveAgentControllerError, EnclaveCreateOptions};

#[derive(Debug, Default, TypedBuilder)]
pub struct EnclaveController {
    pub options: EnclaveCreateOptions,
}

impl EnclaveController {
    pub fn new(options: EnclaveCreateOptions) -> Self {
        Self { options }
    }

    pub async fn start_enclave(&self) -> Result<(), EnclaveAgentControllerError> {
        let output = Command::new("nitro-cli")
            .args([
                "run-enclave",
                "--enclave-name",
                &self.options.enclave_name,
                "--enclave-cid",
                &self.options.enclave_cid.to_string(),
                "--eif-path",
                &self.options.enclave_elf_file,
                "--cpu-count",
                &self.options.enclave_cpu_count.to_string(),
                "--memory",
                &self.options.enclave_memory_size.to_string(),
            ])
            .output()
            .await?;
        if output.status.success() {
            Ok(())
        } else {
            Err(EnclaveAgentControllerError::StartEnclaveError(
                String::from_utf8_lossy(&output.stderr).to_string(),
            ))
        }
    }

    pub async fn stop_enclave(&self) -> Result<(), EnclaveAgentControllerError> {
        let output = Command::new("nitro-cli")
            .args(["terminate-enclave", "--enclave-name", &self.options.enclave_name])
            .output()
            .await?;
        if output.status.success() {
            Ok(())
        } else {
            Err(EnclaveAgentControllerError::StopEnclaveError(
                String::from_utf8_lossy(&output.stderr).to_string(),
            ))
        }
    }

    pub async fn restart_enclave(&self) -> Result<(), EnclaveAgentControllerError> {
        let _ = self.stop_enclave().await.map_err(|e| {
            warn!("stop enclave meet error: {}", e);
        });
        self.start_enclave().await?;
        Ok(())
    }
}
