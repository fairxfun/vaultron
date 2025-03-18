use enclave_utils::time::current_timestamp;

#[tokio::test]
pub async fn test_current_timestamp() {
    let current_timestamp = current_timestamp();
    assert!(current_timestamp > 1741916213000);
    assert!(current_timestamp < 5213122606000);
}
