use tokio::sync::RwLock;

#[derive(Debug)]
struct RawEnclaveData {
    pub pcrs: Vec<String>,
}

impl Default for RawEnclaveData {
    fn default() -> Self {
        Self { pcrs: vec![] }
    }
}

#[derive(Debug)]
pub struct EnclaveData {
    raw: RwLock<RawEnclaveData>,
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

    pub async fn is_initialized(&self) -> bool {
        let raw = self.raw.read().await;
        !raw.pcrs.is_empty()
    }
}
