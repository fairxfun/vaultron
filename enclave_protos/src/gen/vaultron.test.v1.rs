// @generated
#[derive(typed_builder::TypedBuilder)]
#[builder(field_defaults(default, setter(into)))]
#[derive(enclave_protos_macros::ProtoBuilder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TestRequest {
    #[prost(string, tag="1")]
    pub message: ::prost::alloc::string::String,
}
#[derive(typed_builder::TypedBuilder)]
#[builder(field_defaults(default, setter(into)))]
#[derive(enclave_protos_macros::ProtoBuilder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TestResponse {
    #[prost(string, tag="1")]
    pub message: ::prost::alloc::string::String,
}
/// Encoded file descriptor set for the `vaultron.test.v1` package
pub const FILE_DESCRIPTOR_SET: &[u8] = &[
    0x0a, 0xfa, 0x04, 0x0a, 0x1e, 0x76, 0x61, 0x75, 0x6c, 0x74, 0x72, 0x6f, 0x6e, 0x2f, 0x74, 0x65,
    0x73, 0x74, 0x2f, 0x76, 0x31, 0x2f, 0x73, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x2e, 0x70, 0x72,
    0x6f, 0x74, 0x6f, 0x12, 0x10, 0x76, 0x61, 0x75, 0x6c, 0x74, 0x72, 0x6f, 0x6e, 0x2e, 0x74, 0x65,
    0x73, 0x74, 0x2e, 0x76, 0x31, 0x22, 0x27, 0x0a, 0x0b, 0x54, 0x65, 0x73, 0x74, 0x52, 0x65, 0x71,
    0x75, 0x65, 0x73, 0x74, 0x12, 0x18, 0x0a, 0x07, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x18,
    0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x07, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x22, 0x28,
    0x0a, 0x0c, 0x54, 0x65, 0x73, 0x74, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x18,
    0x0a, 0x07, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52,
    0x07, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x32, 0x56, 0x0a, 0x0b, 0x54, 0x65, 0x73, 0x74,
    0x53, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x12, 0x47, 0x0a, 0x04, 0x54, 0x65, 0x73, 0x74, 0x12,
    0x1d, 0x2e, 0x76, 0x61, 0x75, 0x6c, 0x74, 0x72, 0x6f, 0x6e, 0x2e, 0x74, 0x65, 0x73, 0x74, 0x2e,
    0x76, 0x31, 0x2e, 0x54, 0x65, 0x73, 0x74, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x1a, 0x1e,
    0x2e, 0x76, 0x61, 0x75, 0x6c, 0x74, 0x72, 0x6f, 0x6e, 0x2e, 0x74, 0x65, 0x73, 0x74, 0x2e, 0x76,
    0x31, 0x2e, 0x54, 0x65, 0x73, 0x74, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x22, 0x00,
    0x42, 0x86, 0x01, 0x0a, 0x14, 0x63, 0x6f, 0x6d, 0x2e, 0x76, 0x61, 0x75, 0x6c, 0x74, 0x72, 0x6f,
    0x6e, 0x2e, 0x74, 0x65, 0x73, 0x74, 0x2e, 0x76, 0x31, 0x42, 0x0c, 0x53, 0x65, 0x72, 0x76, 0x69,
    0x63, 0x65, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x50, 0x01, 0xa2, 0x02, 0x03, 0x56, 0x54, 0x58, 0xaa,
    0x02, 0x10, 0x56, 0x61, 0x75, 0x6c, 0x74, 0x72, 0x6f, 0x6e, 0x2e, 0x54, 0x65, 0x73, 0x74, 0x2e,
    0x56, 0x31, 0xca, 0x02, 0x10, 0x56, 0x61, 0x75, 0x6c, 0x74, 0x72, 0x6f, 0x6e, 0x5c, 0x54, 0x65,
    0x73, 0x74, 0x5c, 0x56, 0x31, 0xe2, 0x02, 0x1c, 0x56, 0x61, 0x75, 0x6c, 0x74, 0x72, 0x6f, 0x6e,
    0x5c, 0x54, 0x65, 0x73, 0x74, 0x5c, 0x56, 0x31, 0x5c, 0x47, 0x50, 0x42, 0x4d, 0x65, 0x74, 0x61,
    0x64, 0x61, 0x74, 0x61, 0xea, 0x02, 0x12, 0x56, 0x61, 0x75, 0x6c, 0x74, 0x72, 0x6f, 0x6e, 0x3a,
    0x3a, 0x54, 0x65, 0x73, 0x74, 0x3a, 0x3a, 0x56, 0x31, 0x4a, 0x89, 0x02, 0x0a, 0x06, 0x12, 0x04,
    0x00, 0x00, 0x0e, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x0c, 0x12, 0x03, 0x00, 0x00, 0x12, 0x0a, 0x08,
    0x0a, 0x01, 0x02, 0x12, 0x03, 0x02, 0x00, 0x19, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04,
    0x04, 0x00, 0x06, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x04, 0x08, 0x13,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x05, 0x02, 0x15, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x05, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x05, 0x09, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x00, 0x03, 0x12, 0x03, 0x05, 0x13, 0x14, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x01, 0x12, 0x04, 0x08,
    0x00, 0x0a, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01, 0x12, 0x03, 0x08, 0x08, 0x14, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x00, 0x12, 0x03, 0x09, 0x02, 0x15, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x00, 0x05, 0x12, 0x03, 0x09, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x00, 0x01, 0x12, 0x03, 0x09, 0x09, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00,
    0x03, 0x12, 0x03, 0x09, 0x13, 0x14, 0x0a, 0x0a, 0x0a, 0x02, 0x06, 0x00, 0x12, 0x04, 0x0c, 0x00,
    0x0e, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x06, 0x00, 0x01, 0x12, 0x03, 0x0c, 0x08, 0x13, 0x0a, 0x0b,
    0x0a, 0x04, 0x06, 0x00, 0x02, 0x00, 0x12, 0x03, 0x0d, 0x02, 0x31, 0x0a, 0x0c, 0x0a, 0x05, 0x06,
    0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x0d, 0x06, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02,
    0x00, 0x02, 0x12, 0x03, 0x0d, 0x0b, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x00, 0x03,
    0x12, 0x03, 0x0d, 0x21, 0x2d, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
];
include!("vaultron.test.v1.tonic.rs");
include!("vaultron.test.v1.serde.rs");
// @@protoc_insertion_point(module)