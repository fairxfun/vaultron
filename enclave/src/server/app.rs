use crate::{common::enclave_trace_init, common::EnclaveError, kms::create_kms_handler};
use enclave_kmstool::KmsToolTrait;
use log::info;
use std::{
    io::{Read, Write},
    sync::Arc,
};
use vsock::{get_local_cid, VsockListener, VsockStream};

const ENCLAVE_PORT: u32 = 5001;
const BUF_MAX_LEN: usize = 8192;

pub struct EnclaveServer<K: KmsToolTrait> {
    pub kms_client: Arc<K>,
}

pub fn start_server() -> Result<(), EnclaveError> {
    enclave_trace_init("info")?;
    let cid = get_local_cid()?;
    info!("start enclave with cid {} port {}", cid, ENCLAVE_PORT);
    let _ = create_kms_handler();
    let listener = VsockListener::bind_with_cid_port(cid, ENCLAVE_PORT)?;
    loop {
        let (stream, _) = listener.accept()?;
        info!("Client connected!");
        handle_client(stream)?;
    }
}

fn handle_client(mut stream: VsockStream) -> std::io::Result<()> {
    let mut buf = vec![0u8; BUF_MAX_LEN];

    let size = stream.read(&mut buf)?;
    info!("Received: {}", String::from_utf8_lossy(&buf[..size]));

    let response = "Hello from Enclave!";
    stream.write_all(response.as_bytes())?;
    stream.flush()?;

    Ok(())
}
