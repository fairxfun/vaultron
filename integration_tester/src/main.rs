use anyhow::Result;
use integration_tester::{
    enclave::enclave_test, enclave_agent::enclave_agent_test, integration_trace_init, EnclaveTesterError,
    MessageHandler,
};
use log::info;
use std::env;

#[tokio::main]
async fn main() -> Result<(), EnclaveTesterError> {
    integration_trace_init("info").expect("Failed to initialize trace");
    let args: Vec<String> = env::args().collect();
    let pcr0 = if args.len() == 2 {
        let pcr0_hex = &args[1];
        if pcr0_hex.len() != 96 {
            return Err(EnclaveTesterError::InvalidPCR0Error);
        }
        hex::decode(pcr0_hex).expect("Invalid PCR0")
    } else {
        vec![0u8; 48]
    };

    start_test(pcr0).await.expect("Test failed");
    info!("Test completed successfully");
    Ok(())
}

async fn start_test(pcr0: Vec<u8>) -> Result<(), EnclaveTesterError> {
    let mut handler = MessageHandler::new(pcr0.clone()).await?;
    enclave_agent_test(&mut handler).await?;
    enclave_test(&mut handler, pcr0).await?;
    Ok(())
}
