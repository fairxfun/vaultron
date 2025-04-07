use integration_tester::{start_test, EnclaveTesterError};
use std::env;

#[tokio::main]
async fn main() -> Result<(), EnclaveTesterError> {
    let args: Vec<String> = env::args().collect();
    let pcr0 = if args.len() == 2 {
        let pcr0_hex = &args[1];
        if pcr0_hex.len() != 96 {
            return Err(EnclaveTesterError::InvalidPCR0Error);
        }
        hex::decode(pcr0_hex).map_err(|_| EnclaveTesterError::InvalidPCR0Error)?
    } else {
        vec![0u8; 48]
    };

    if pcr0.len() != 48 {
        return Err(EnclaveTesterError::InvalidPCR0Error);
    }

    let result = start_test(pcr0).await;
    match result {
        Ok(_) => {
            println!("Test completed successfully");
            Ok(())
        }
        Err(e) => {
            eprintln!("Test failed: {}", e);
            Err(e)
        }
    }
}
