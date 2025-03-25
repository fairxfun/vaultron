use tokio::sync::RwLock;

#[derive(Debug, Default)]
struct RawEnclaveData {
    pub pcrs: Vec<String>,
}

#[derive(Debug)]
pub struct EnclaveData {
    raw: RwLock<RawEnclaveData>,
}

impl Default for EnclaveData {
    fn default() -> Self {
        Self::new()
    }
}

impl EnclaveData {
    pub fn new() -> Self {
        Self {
            raw: RwLock::new(RawEnclaveData::default()),
        }
    }

    pub async fn set_pcrs(&self, pcrs: Vec<String>) {
        let mut raw = self.raw.write().await;
        raw.pcrs = pcrs;
    }

    pub async fn get_pcrs(&self) -> Vec<String> {
        let raw = self.raw.read().await;
        raw.pcrs.clone()
    }

    pub async fn get_pcr0(&self) -> String {
        let pcrs = self.get_pcrs().await;
        pcrs.first().unwrap_or(&"".to_string()).clone()
    }

    pub async fn is_initialized(&self) -> bool {
        let raw = self.raw.read().await;
        !raw.pcrs.is_empty()
    }
}
