use enclave_utils::address::{
    ethers_address_from_bytes, ethers_address_from_string, ethers_address_to_bytes, ethers_address_to_string,
    string_address_from_bytes, string_address_to_bytes,
};

#[tokio::test]
pub async fn test_ethers_address_from_string() {
    let address_str = "0x7E2202581b1d8FceaB95A57c3766e0E6dA8c1a72";
    let address1 = ethers_address_from_string(address_str).unwrap();
    assert_eq!(ethers_address_to_string(&address1), address_str);

    let address_bytes = [
        126, 34, 2, 88, 27, 29, 143, 206, 171, 149, 165, 124, 55, 102, 224, 230, 218, 140, 26, 114,
    ];
    let address2 = ethers_address_from_bytes(address_bytes).unwrap();
    assert_eq!(address1, address2);
    assert_eq!(ethers_address_to_bytes(&address2), address_bytes);

    let address_str2 = string_address_from_bytes(address_bytes).unwrap();
    assert_eq!(address_str2, address_str);

    let address_bytes2 = string_address_to_bytes(address_str).unwrap();
    assert_eq!(address_bytes2, address_bytes);

    let address_bytes = [126];
    let address3 = ethers_address_from_bytes(address_bytes);
    assert!(address3.is_err());
}
