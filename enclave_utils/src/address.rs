use ethers_core::types::Address;
use ethers_core::utils::to_checksum;
use std::str::FromStr;

pub fn ethers_address_from_string<S>(address: S) -> Result<Address, rustc_hex::FromHexError>
where
    S: AsRef<str>,
{
    Address::from_str(address.as_ref())
}

pub fn ethers_address_from_bytes<B>(address: B) -> Result<Address, rustc_hex::FromHexError>
where
    B: AsRef<[u8]>,
{
    if address.as_ref().len() == 20 {
        Ok(Address::from_slice(address.as_ref()))
    } else {
        Err(rustc_hex::FromHexError::InvalidHexLength)
    }
}

pub fn ethers_address_to_string(address: &Address) -> String {
    to_checksum(address, None)
}

pub fn ethers_address_to_bytes(address: &Address) -> Vec<u8> {
    address.as_bytes().to_vec()
}

pub fn string_address_from_bytes<B>(address: B) -> Result<String, rustc_hex::FromHexError>
where
    B: AsRef<[u8]>,
{
    Ok(ethers_address_to_string(&ethers_address_from_bytes(address)?))
}

pub fn string_address_to_bytes<S>(address: S) -> Result<Vec<u8>, rustc_hex::FromHexError>
where
    S: AsRef<str>,
{
    Ok(ethers_address_to_bytes(&ethers_address_from_string(address)?))
}
