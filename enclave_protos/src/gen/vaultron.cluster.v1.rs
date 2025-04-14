// @generated
#[derive(typed_builder::TypedBuilder)]
#[builder(field_defaults(default, setter(into)))]
#[derive(enclave_protos_macros::ProtoBuilder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClusterAttributes {
    #[prost(bytes="vec", tag="1")]
    pub enclave_pcr0: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="2")]
    pub cluster_public_key: ::prost::alloc::vec::Vec<u8>,
}
#[derive(typed_builder::TypedBuilder)]
#[builder(field_defaults(default, setter(into)))]
#[derive(enclave_protos_macros::ProtoBuilder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateUserWalletParams {
    #[prost(message, optional, tag="1")]
    pub attributes: ::core::option::Option<ClusterAttributes>,
    #[prost(bytes="vec", tag="2")]
    pub user_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="3")]
    pub user_public_key: ::prost::alloc::vec::Vec<u8>,
    #[prost(enumeration="SignatureType", tag="4")]
    pub signature_type: i32,
    #[prost(bytes="vec", tag="5")]
    pub message: ::prost::alloc::vec::Vec<u8>,
}
#[derive(typed_builder::TypedBuilder)]
#[builder(field_defaults(default, setter(into)))]
#[derive(enclave_protos_macros::ProtoBuilder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateUserWalletRequest {
    #[prost(message, optional, tag="1")]
    pub params: ::core::option::Option<CreateUserWalletParams>,
    #[prost(bytes="vec", tag="2")]
    pub signature: ::prost::alloc::vec::Vec<u8>,
}
#[derive(typed_builder::TypedBuilder)]
#[builder(field_defaults(default, setter(into)))]
#[derive(enclave_protos_macros::ProtoBuilder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateUserWalletResponse {
    #[prost(bytes="vec", tag="1")]
    pub wallet_encrypted_data: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="2")]
    pub eth_public_key: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="3")]
    pub eth_encrypted_data: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="4")]
    pub solana_public_key: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="5")]
    pub solana_encrypted_data: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="6")]
    pub sui_public_key: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="7")]
    pub sui_encrypted_data: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SignatureType {
    Unspecified = 0,
    Fairx = 1,
    WalletBitcoin = 2,
    WalletEth = 3,
    WalletSolana = 4,
    WalletSui = 5,
}
impl SignatureType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            SignatureType::Unspecified => "SIGNATURE_TYPE_UNSPECIFIED",
            SignatureType::Fairx => "SIGNATURE_TYPE_FAIRX",
            SignatureType::WalletBitcoin => "SIGNATURE_TYPE_WALLET_BITCOIN",
            SignatureType::WalletEth => "SIGNATURE_TYPE_WALLET_ETH",
            SignatureType::WalletSolana => "SIGNATURE_TYPE_WALLET_SOLANA",
            SignatureType::WalletSui => "SIGNATURE_TYPE_WALLET_SUI",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "SIGNATURE_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "SIGNATURE_TYPE_FAIRX" => Some(Self::Fairx),
            "SIGNATURE_TYPE_WALLET_BITCOIN" => Some(Self::WalletBitcoin),
            "SIGNATURE_TYPE_WALLET_ETH" => Some(Self::WalletEth),
            "SIGNATURE_TYPE_WALLET_SOLANA" => Some(Self::WalletSolana),
            "SIGNATURE_TYPE_WALLET_SUI" => Some(Self::WalletSui),
            _ => None,
        }
    }
}
/// Encoded file descriptor set for the `vaultron.cluster.v1` package
pub const FILE_DESCRIPTOR_SET: &[u8] = &[
    0x0a, 0xe1, 0x03, 0x0a, 0x20, 0x76, 0x61, 0x75, 0x6c, 0x74, 0x72, 0x6f, 0x6e, 0x2f, 0x63, 0x6c,
    0x75, 0x73, 0x74, 0x65, 0x72, 0x2f, 0x76, 0x31, 0x2f, 0x63, 0x6f, 0x6d, 0x6d, 0x6f, 0x6e, 0x2e,
    0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x13, 0x76, 0x61, 0x75, 0x6c, 0x74, 0x72, 0x6f, 0x6e, 0x2e,
    0x63, 0x6c, 0x75, 0x73, 0x74, 0x65, 0x72, 0x2e, 0x76, 0x31, 0x22, 0x64, 0x0a, 0x11, 0x43, 0x6c,
    0x75, 0x73, 0x74, 0x65, 0x72, 0x41, 0x74, 0x74, 0x72, 0x69, 0x62, 0x75, 0x74, 0x65, 0x73, 0x12,
    0x21, 0x0a, 0x0c, 0x65, 0x6e, 0x63, 0x6c, 0x61, 0x76, 0x65, 0x5f, 0x70, 0x63, 0x72, 0x30, 0x18,
    0x01, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x0b, 0x65, 0x6e, 0x63, 0x6c, 0x61, 0x76, 0x65, 0x50, 0x63,
    0x72, 0x30, 0x12, 0x2c, 0x0a, 0x12, 0x63, 0x6c, 0x75, 0x73, 0x74, 0x65, 0x72, 0x5f, 0x70, 0x75,
    0x62, 0x6c, 0x69, 0x63, 0x5f, 0x6b, 0x65, 0x79, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x10,
    0x63, 0x6c, 0x75, 0x73, 0x74, 0x65, 0x72, 0x50, 0x75, 0x62, 0x6c, 0x69, 0x63, 0x4b, 0x65, 0x79,
    0x42, 0x94, 0x01, 0x0a, 0x17, 0x63, 0x6f, 0x6d, 0x2e, 0x76, 0x61, 0x75, 0x6c, 0x74, 0x72, 0x6f,
    0x6e, 0x2e, 0x63, 0x6c, 0x75, 0x73, 0x74, 0x65, 0x72, 0x2e, 0x76, 0x31, 0x42, 0x0b, 0x43, 0x6f,
    0x6d, 0x6d, 0x6f, 0x6e, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x50, 0x01, 0xa2, 0x02, 0x03, 0x56, 0x43,
    0x58, 0xaa, 0x02, 0x13, 0x56, 0x61, 0x75, 0x6c, 0x74, 0x72, 0x6f, 0x6e, 0x2e, 0x43, 0x6c, 0x75,
    0x73, 0x74, 0x65, 0x72, 0x2e, 0x56, 0x31, 0xca, 0x02, 0x13, 0x56, 0x61, 0x75, 0x6c, 0x74, 0x72,
    0x6f, 0x6e, 0x5c, 0x43, 0x6c, 0x75, 0x73, 0x74, 0x65, 0x72, 0x5c, 0x56, 0x31, 0xe2, 0x02, 0x1f,
    0x56, 0x61, 0x75, 0x6c, 0x74, 0x72, 0x6f, 0x6e, 0x5c, 0x43, 0x6c, 0x75, 0x73, 0x74, 0x65, 0x72,
    0x5c, 0x56, 0x31, 0x5c, 0x47, 0x50, 0x42, 0x4d, 0x65, 0x74, 0x61, 0x64, 0x61, 0x74, 0x61, 0xea,
    0x02, 0x15, 0x56, 0x61, 0x75, 0x6c, 0x74, 0x72, 0x6f, 0x6e, 0x3a, 0x3a, 0x43, 0x6c, 0x75, 0x73,
    0x74, 0x65, 0x72, 0x3a, 0x3a, 0x56, 0x31, 0x4a, 0xa2, 0x01, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00,
    0x07, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x0c, 0x12, 0x03, 0x00, 0x00, 0x12, 0x0a, 0x08, 0x0a, 0x01,
    0x02, 0x12, 0x03, 0x02, 0x00, 0x1c, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x04, 0x00,
    0x07, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x04, 0x08, 0x19, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x05, 0x02, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x05, 0x02, 0x07, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x00, 0x01, 0x12, 0x03, 0x05, 0x08, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03,
    0x12, 0x03, 0x05, 0x1d, 0x1e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x06,
    0x02, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x05, 0x12, 0x03, 0x06, 0x02, 0x07,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x06, 0x08, 0x1a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x06, 0x1d, 0x1e, 0x62, 0x06, 0x70, 0x72,
    0x6f, 0x74, 0x6f, 0x33, 0x0a, 0xa9, 0x12, 0x0a, 0x20, 0x76, 0x61, 0x75, 0x6c, 0x74, 0x72, 0x6f,
    0x6e, 0x2f, 0x63, 0x6c, 0x75, 0x73, 0x74, 0x65, 0x72, 0x2f, 0x76, 0x31, 0x2f, 0x77, 0x61, 0x6c,
    0x6c, 0x65, 0x74, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x13, 0x76, 0x61, 0x75, 0x6c, 0x74,
    0x72, 0x6f, 0x6e, 0x2e, 0x63, 0x6c, 0x75, 0x73, 0x74, 0x65, 0x72, 0x2e, 0x76, 0x31, 0x1a, 0x20,
    0x76, 0x61, 0x75, 0x6c, 0x74, 0x72, 0x6f, 0x6e, 0x2f, 0x63, 0x6c, 0x75, 0x73, 0x74, 0x65, 0x72,
    0x2f, 0x76, 0x31, 0x2f, 0x63, 0x6f, 0x6d, 0x6d, 0x6f, 0x6e, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f,
    0x22, 0x86, 0x02, 0x0a, 0x16, 0x43, 0x72, 0x65, 0x61, 0x74, 0x65, 0x55, 0x73, 0x65, 0x72, 0x57,
    0x61, 0x6c, 0x6c, 0x65, 0x74, 0x50, 0x61, 0x72, 0x61, 0x6d, 0x73, 0x12, 0x46, 0x0a, 0x0a, 0x61,
    0x74, 0x74, 0x72, 0x69, 0x62, 0x75, 0x74, 0x65, 0x73, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32,
    0x26, 0x2e, 0x76, 0x61, 0x75, 0x6c, 0x74, 0x72, 0x6f, 0x6e, 0x2e, 0x63, 0x6c, 0x75, 0x73, 0x74,
    0x65, 0x72, 0x2e, 0x76, 0x31, 0x2e, 0x43, 0x6c, 0x75, 0x73, 0x74, 0x65, 0x72, 0x41, 0x74, 0x74,
    0x72, 0x69, 0x62, 0x75, 0x74, 0x65, 0x73, 0x52, 0x0a, 0x61, 0x74, 0x74, 0x72, 0x69, 0x62, 0x75,
    0x74, 0x65, 0x73, 0x12, 0x17, 0x0a, 0x07, 0x75, 0x73, 0x65, 0x72, 0x5f, 0x69, 0x64, 0x18, 0x02,
    0x20, 0x01, 0x28, 0x0c, 0x52, 0x06, 0x75, 0x73, 0x65, 0x72, 0x49, 0x64, 0x12, 0x26, 0x0a, 0x0f,
    0x75, 0x73, 0x65, 0x72, 0x5f, 0x70, 0x75, 0x62, 0x6c, 0x69, 0x63, 0x5f, 0x6b, 0x65, 0x79, 0x18,
    0x03, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x0d, 0x75, 0x73, 0x65, 0x72, 0x50, 0x75, 0x62, 0x6c, 0x69,
    0x63, 0x4b, 0x65, 0x79, 0x12, 0x49, 0x0a, 0x0e, 0x73, 0x69, 0x67, 0x6e, 0x61, 0x74, 0x75, 0x72,
    0x65, 0x5f, 0x74, 0x79, 0x70, 0x65, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x22, 0x2e, 0x76,
    0x61, 0x75, 0x6c, 0x74, 0x72, 0x6f, 0x6e, 0x2e, 0x63, 0x6c, 0x75, 0x73, 0x74, 0x65, 0x72, 0x2e,
    0x76, 0x31, 0x2e, 0x53, 0x69, 0x67, 0x6e, 0x61, 0x74, 0x75, 0x72, 0x65, 0x54, 0x79, 0x70, 0x65,
    0x52, 0x0d, 0x73, 0x69, 0x67, 0x6e, 0x61, 0x74, 0x75, 0x72, 0x65, 0x54, 0x79, 0x70, 0x65, 0x12,
    0x18, 0x0a, 0x07, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x18, 0x05, 0x20, 0x01, 0x28, 0x0c,
    0x52, 0x07, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x22, 0x7c, 0x0a, 0x17, 0x43, 0x72, 0x65,
    0x61, 0x74, 0x65, 0x55, 0x73, 0x65, 0x72, 0x57, 0x61, 0x6c, 0x6c, 0x65, 0x74, 0x52, 0x65, 0x71,
    0x75, 0x65, 0x73, 0x74, 0x12, 0x43, 0x0a, 0x06, 0x70, 0x61, 0x72, 0x61, 0x6d, 0x73, 0x18, 0x01,
    0x20, 0x01, 0x28, 0x0b, 0x32, 0x2b, 0x2e, 0x76, 0x61, 0x75, 0x6c, 0x74, 0x72, 0x6f, 0x6e, 0x2e,
    0x63, 0x6c, 0x75, 0x73, 0x74, 0x65, 0x72, 0x2e, 0x76, 0x31, 0x2e, 0x43, 0x72, 0x65, 0x61, 0x74,
    0x65, 0x55, 0x73, 0x65, 0x72, 0x57, 0x61, 0x6c, 0x6c, 0x65, 0x74, 0x50, 0x61, 0x72, 0x61, 0x6d,
    0x73, 0x52, 0x06, 0x70, 0x61, 0x72, 0x61, 0x6d, 0x73, 0x12, 0x1c, 0x0a, 0x09, 0x73, 0x69, 0x67,
    0x6e, 0x61, 0x74, 0x75, 0x72, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x09, 0x73, 0x69,
    0x67, 0x6e, 0x61, 0x74, 0x75, 0x72, 0x65, 0x22, 0xd6, 0x02, 0x0a, 0x18, 0x43, 0x72, 0x65, 0x61,
    0x74, 0x65, 0x55, 0x73, 0x65, 0x72, 0x57, 0x61, 0x6c, 0x6c, 0x65, 0x74, 0x52, 0x65, 0x73, 0x70,
    0x6f, 0x6e, 0x73, 0x65, 0x12, 0x32, 0x0a, 0x15, 0x77, 0x61, 0x6c, 0x6c, 0x65, 0x74, 0x5f, 0x65,
    0x6e, 0x63, 0x72, 0x79, 0x70, 0x74, 0x65, 0x64, 0x5f, 0x64, 0x61, 0x74, 0x61, 0x18, 0x01, 0x20,
    0x01, 0x28, 0x0c, 0x52, 0x13, 0x77, 0x61, 0x6c, 0x6c, 0x65, 0x74, 0x45, 0x6e, 0x63, 0x72, 0x79,
    0x70, 0x74, 0x65, 0x64, 0x44, 0x61, 0x74, 0x61, 0x12, 0x24, 0x0a, 0x0e, 0x65, 0x74, 0x68, 0x5f,
    0x70, 0x75, 0x62, 0x6c, 0x69, 0x63, 0x5f, 0x6b, 0x65, 0x79, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0c,
    0x52, 0x0c, 0x65, 0x74, 0x68, 0x50, 0x75, 0x62, 0x6c, 0x69, 0x63, 0x4b, 0x65, 0x79, 0x12, 0x2c,
    0x0a, 0x12, 0x65, 0x74, 0x68, 0x5f, 0x65, 0x6e, 0x63, 0x72, 0x79, 0x70, 0x74, 0x65, 0x64, 0x5f,
    0x64, 0x61, 0x74, 0x61, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x10, 0x65, 0x74, 0x68, 0x45,
    0x6e, 0x63, 0x72, 0x79, 0x70, 0x74, 0x65, 0x64, 0x44, 0x61, 0x74, 0x61, 0x12, 0x2a, 0x0a, 0x11,
    0x73, 0x6f, 0x6c, 0x61, 0x6e, 0x61, 0x5f, 0x70, 0x75, 0x62, 0x6c, 0x69, 0x63, 0x5f, 0x6b, 0x65,
    0x79, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x0f, 0x73, 0x6f, 0x6c, 0x61, 0x6e, 0x61, 0x50,
    0x75, 0x62, 0x6c, 0x69, 0x63, 0x4b, 0x65, 0x79, 0x12, 0x32, 0x0a, 0x15, 0x73, 0x6f, 0x6c, 0x61,
    0x6e, 0x61, 0x5f, 0x65, 0x6e, 0x63, 0x72, 0x79, 0x70, 0x74, 0x65, 0x64, 0x5f, 0x64, 0x61, 0x74,
    0x61, 0x18, 0x05, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x13, 0x73, 0x6f, 0x6c, 0x61, 0x6e, 0x61, 0x45,
    0x6e, 0x63, 0x72, 0x79, 0x70, 0x74, 0x65, 0x64, 0x44, 0x61, 0x74, 0x61, 0x12, 0x24, 0x0a, 0x0e,
    0x73, 0x75, 0x69, 0x5f, 0x70, 0x75, 0x62, 0x6c, 0x69, 0x63, 0x5f, 0x6b, 0x65, 0x79, 0x18, 0x06,
    0x20, 0x01, 0x28, 0x0c, 0x52, 0x0c, 0x73, 0x75, 0x69, 0x50, 0x75, 0x62, 0x6c, 0x69, 0x63, 0x4b,
    0x65, 0x79, 0x12, 0x2c, 0x0a, 0x12, 0x73, 0x75, 0x69, 0x5f, 0x65, 0x6e, 0x63, 0x72, 0x79, 0x70,
    0x74, 0x65, 0x64, 0x5f, 0x64, 0x61, 0x74, 0x61, 0x18, 0x07, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x10,
    0x73, 0x75, 0x69, 0x45, 0x6e, 0x63, 0x72, 0x79, 0x70, 0x74, 0x65, 0x64, 0x44, 0x61, 0x74, 0x61,
    0x2a, 0xcc, 0x01, 0x0a, 0x0d, 0x53, 0x69, 0x67, 0x6e, 0x61, 0x74, 0x75, 0x72, 0x65, 0x54, 0x79,
    0x70, 0x65, 0x12, 0x1e, 0x0a, 0x1a, 0x53, 0x49, 0x47, 0x4e, 0x41, 0x54, 0x55, 0x52, 0x45, 0x5f,
    0x54, 0x59, 0x50, 0x45, 0x5f, 0x55, 0x4e, 0x53, 0x50, 0x45, 0x43, 0x49, 0x46, 0x49, 0x45, 0x44,
    0x10, 0x00, 0x12, 0x18, 0x0a, 0x14, 0x53, 0x49, 0x47, 0x4e, 0x41, 0x54, 0x55, 0x52, 0x45, 0x5f,
    0x54, 0x59, 0x50, 0x45, 0x5f, 0x46, 0x41, 0x49, 0x52, 0x58, 0x10, 0x01, 0x12, 0x21, 0x0a, 0x1d,
    0x53, 0x49, 0x47, 0x4e, 0x41, 0x54, 0x55, 0x52, 0x45, 0x5f, 0x54, 0x59, 0x50, 0x45, 0x5f, 0x57,
    0x41, 0x4c, 0x4c, 0x45, 0x54, 0x5f, 0x42, 0x49, 0x54, 0x43, 0x4f, 0x49, 0x4e, 0x10, 0x02, 0x12,
    0x1d, 0x0a, 0x19, 0x53, 0x49, 0x47, 0x4e, 0x41, 0x54, 0x55, 0x52, 0x45, 0x5f, 0x54, 0x59, 0x50,
    0x45, 0x5f, 0x57, 0x41, 0x4c, 0x4c, 0x45, 0x54, 0x5f, 0x45, 0x54, 0x48, 0x10, 0x03, 0x12, 0x20,
    0x0a, 0x1c, 0x53, 0x49, 0x47, 0x4e, 0x41, 0x54, 0x55, 0x52, 0x45, 0x5f, 0x54, 0x59, 0x50, 0x45,
    0x5f, 0x57, 0x41, 0x4c, 0x4c, 0x45, 0x54, 0x5f, 0x53, 0x4f, 0x4c, 0x41, 0x4e, 0x41, 0x10, 0x04,
    0x12, 0x1d, 0x0a, 0x19, 0x53, 0x49, 0x47, 0x4e, 0x41, 0x54, 0x55, 0x52, 0x45, 0x5f, 0x54, 0x59,
    0x50, 0x45, 0x5f, 0x57, 0x41, 0x4c, 0x4c, 0x45, 0x54, 0x5f, 0x53, 0x55, 0x49, 0x10, 0x05, 0x42,
    0x94, 0x01, 0x0a, 0x17, 0x63, 0x6f, 0x6d, 0x2e, 0x76, 0x61, 0x75, 0x6c, 0x74, 0x72, 0x6f, 0x6e,
    0x2e, 0x63, 0x6c, 0x75, 0x73, 0x74, 0x65, 0x72, 0x2e, 0x76, 0x31, 0x42, 0x0b, 0x57, 0x61, 0x6c,
    0x6c, 0x65, 0x74, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x50, 0x01, 0xa2, 0x02, 0x03, 0x56, 0x43, 0x58,
    0xaa, 0x02, 0x13, 0x56, 0x61, 0x75, 0x6c, 0x74, 0x72, 0x6f, 0x6e, 0x2e, 0x43, 0x6c, 0x75, 0x73,
    0x74, 0x65, 0x72, 0x2e, 0x56, 0x31, 0xca, 0x02, 0x13, 0x56, 0x61, 0x75, 0x6c, 0x74, 0x72, 0x6f,
    0x6e, 0x5c, 0x43, 0x6c, 0x75, 0x73, 0x74, 0x65, 0x72, 0x5c, 0x56, 0x31, 0xe2, 0x02, 0x1f, 0x56,
    0x61, 0x75, 0x6c, 0x74, 0x72, 0x6f, 0x6e, 0x5c, 0x43, 0x6c, 0x75, 0x73, 0x74, 0x65, 0x72, 0x5c,
    0x56, 0x31, 0x5c, 0x47, 0x50, 0x42, 0x4d, 0x65, 0x74, 0x61, 0x64, 0x61, 0x74, 0x61, 0xea, 0x02,
    0x15, 0x56, 0x61, 0x75, 0x6c, 0x74, 0x72, 0x6f, 0x6e, 0x3a, 0x3a, 0x43, 0x6c, 0x75, 0x73, 0x74,
    0x65, 0x72, 0x3a, 0x3a, 0x56, 0x31, 0x4a, 0xff, 0x08, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x24,
    0x01, 0x0a, 0x08, 0x0a, 0x01, 0x0c, 0x12, 0x03, 0x00, 0x00, 0x12, 0x0a, 0x08, 0x0a, 0x01, 0x02,
    0x12, 0x03, 0x02, 0x00, 0x1c, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x00, 0x12, 0x03, 0x04, 0x00, 0x2a,
    0x0a, 0x0a, 0x0a, 0x02, 0x05, 0x00, 0x12, 0x04, 0x06, 0x00, 0x0d, 0x01, 0x0a, 0x0a, 0x0a, 0x03,
    0x05, 0x00, 0x01, 0x12, 0x03, 0x06, 0x05, 0x12, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x00,
    0x12, 0x03, 0x07, 0x02, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03,
    0x07, 0x02, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x00, 0x02, 0x12, 0x03, 0x07, 0x22,
    0x23, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x01, 0x12, 0x03, 0x08, 0x02, 0x24, 0x0a, 0x0c,
    0x0a, 0x05, 0x05, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x08, 0x02, 0x16, 0x0a, 0x0c, 0x0a, 0x05,
    0x05, 0x00, 0x02, 0x01, 0x02, 0x12, 0x03, 0x08, 0x22, 0x23, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00,
    0x02, 0x02, 0x12, 0x03, 0x09, 0x02, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x02, 0x01,
    0x12, 0x03, 0x09, 0x02, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x02, 0x02, 0x12, 0x03,
    0x09, 0x22, 0x23, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x03, 0x12, 0x03, 0x0a, 0x02, 0x24,
    0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x03, 0x01, 0x12, 0x03, 0x0a, 0x02, 0x1b, 0x0a, 0x0c,
    0x0a, 0x05, 0x05, 0x00, 0x02, 0x03, 0x02, 0x12, 0x03, 0x0a, 0x22, 0x23, 0x0a, 0x0b, 0x0a, 0x04,
    0x05, 0x00, 0x02, 0x04, 0x12, 0x03, 0x0b, 0x02, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02,
    0x04, 0x01, 0x12, 0x03, 0x0b, 0x02, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x04, 0x02,
    0x12, 0x03, 0x0b, 0x22, 0x23, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x05, 0x12, 0x03, 0x0c,
    0x02, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x05, 0x01, 0x12, 0x03, 0x0c, 0x02, 0x1b,
    0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x05, 0x02, 0x12, 0x03, 0x0c, 0x22, 0x23, 0x0a, 0x0a,
    0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x0f, 0x00, 0x15, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00,
    0x01, 0x12, 0x03, 0x0f, 0x08, 0x1e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03,
    0x10, 0x04, 0x3e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x06, 0x12, 0x03, 0x10, 0x04,
    0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x10, 0x2a, 0x34, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x10, 0x3c, 0x3d, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x11, 0x04, 0x3e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x01, 0x05, 0x12, 0x03, 0x11, 0x04, 0x09, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01,
    0x01, 0x12, 0x03, 0x11, 0x2a, 0x31, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12,
    0x03, 0x11, 0x3c, 0x3d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x02, 0x12, 0x03, 0x12, 0x04,
    0x3e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x05, 0x12, 0x03, 0x12, 0x04, 0x09, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x12, 0x2a, 0x39, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x02, 0x03, 0x12, 0x03, 0x12, 0x3c, 0x3d, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x00, 0x02, 0x03, 0x12, 0x03, 0x13, 0x04, 0x3e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03,
    0x06, 0x12, 0x03, 0x13, 0x04, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x01, 0x12,
    0x03, 0x13, 0x2a, 0x38, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x03, 0x12, 0x03, 0x13,
    0x3c, 0x3d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x04, 0x12, 0x03, 0x14, 0x04, 0x3e, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x05, 0x12, 0x03, 0x14, 0x04, 0x09, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x04, 0x01, 0x12, 0x03, 0x14, 0x2a, 0x31, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x04, 0x03, 0x12, 0x03, 0x14, 0x3c, 0x3d, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x01, 0x12,
    0x04, 0x17, 0x00, 0x1a, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01, 0x12, 0x03, 0x17, 0x08,
    0x1f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x00, 0x12, 0x03, 0x18, 0x04, 0x29, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x06, 0x12, 0x03, 0x18, 0x04, 0x1a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x18, 0x1b, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x00, 0x03, 0x12, 0x03, 0x18, 0x27, 0x28, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x01,
    0x12, 0x03, 0x19, 0x04, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x05, 0x12, 0x03,
    0x19, 0x04, 0x09, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x01, 0x12, 0x03, 0x19, 0x1b,
    0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x03, 0x12, 0x03, 0x19, 0x27, 0x28, 0x0a,
    0x0a, 0x0a, 0x02, 0x04, 0x02, 0x12, 0x04, 0x1c, 0x00, 0x24, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04,
    0x02, 0x01, 0x12, 0x03, 0x1c, 0x08, 0x20, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x00, 0x12,
    0x03, 0x1d, 0x04, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x05, 0x12, 0x03, 0x1d,
    0x04, 0x09, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x01, 0x12, 0x03, 0x1d, 0x0a, 0x1f,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x03, 0x12, 0x03, 0x1d, 0x22, 0x23, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x02, 0x02, 0x01, 0x12, 0x03, 0x1e, 0x04, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x02, 0x02, 0x01, 0x05, 0x12, 0x03, 0x1e, 0x04, 0x09, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02,
    0x01, 0x01, 0x12, 0x03, 0x1e, 0x0a, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x03,
    0x12, 0x03, 0x1e, 0x22, 0x23, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x02, 0x12, 0x03, 0x1f,
    0x04, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x05, 0x12, 0x03, 0x1f, 0x04, 0x09,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x01, 0x12, 0x03, 0x1f, 0x0a, 0x1c, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x03, 0x12, 0x03, 0x1f, 0x22, 0x23, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x02, 0x02, 0x03, 0x12, 0x03, 0x20, 0x04, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02,
    0x03, 0x05, 0x12, 0x03, 0x20, 0x04, 0x09, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x03, 0x01,
    0x12, 0x03, 0x20, 0x0a, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x03, 0x03, 0x12, 0x03,
    0x20, 0x22, 0x23, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x04, 0x12, 0x03, 0x21, 0x04, 0x24,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x04, 0x05, 0x12, 0x03, 0x21, 0x04, 0x09, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x02, 0x02, 0x04, 0x01, 0x12, 0x03, 0x21, 0x0a, 0x1f, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x02, 0x02, 0x04, 0x03, 0x12, 0x03, 0x21, 0x22, 0x23, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02,
    0x02, 0x05, 0x12, 0x03, 0x22, 0x04, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x05, 0x05,
    0x12, 0x03, 0x22, 0x04, 0x09, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x05, 0x01, 0x12, 0x03,
    0x22, 0x0a, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x05, 0x03, 0x12, 0x03, 0x22, 0x22,
    0x23, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x06, 0x12, 0x03, 0x23, 0x04, 0x24, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x02, 0x02, 0x06, 0x05, 0x12, 0x03, 0x23, 0x04, 0x09, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x02, 0x02, 0x06, 0x01, 0x12, 0x03, 0x23, 0x0a, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02,
    0x02, 0x06, 0x03, 0x12, 0x03, 0x23, 0x22, 0x23, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
];
include!("vaultron.cluster.v1.serde.rs");
// @@protoc_insertion_point(module)