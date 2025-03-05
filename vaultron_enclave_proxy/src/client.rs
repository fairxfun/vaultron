use crate::args::EnclaveArgs;
use crate::error::FairxProxyError;
use std::io::{Read, Write};
use vsock::VsockStream;

const BUF_MAX_LEN: usize = 8192;

pub fn enclave_client(args: EnclaveArgs) -> Result<(), FairxProxyError> {
    let mut stream = VsockStream::connect_with_cid_port(args.cid, args.port)?;
    println!("Connected to Enclave!");

    let message = "Hello, Enclave!";
    stream.write_all(message.as_bytes())?;
    stream.flush()?;

    let mut buf = vec![0u8; BUF_MAX_LEN];
    let size = stream.read(&mut buf)?;
    println!("Response: {}", String::from_utf8_lossy(&buf[..size]));

    Ok(())
}
