// @generated
#[derive(typed_builder::TypedBuilder)]
#[builder(field_defaults(default, setter(into)))]
#[derive(enclave_protos_macros::ProtoBuilder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClientOptions {
    #[prost(string, optional, tag="1")]
    pub host: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint32, optional, tag="2")]
    pub port: ::core::option::Option<u32>,
    #[prost(bool, optional, tag="3")]
    pub is_ssl: ::core::option::Option<bool>,
    #[prost(string, optional, tag="4")]
    pub ssl_cert: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="5")]
    pub ssl_cert_path: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="6")]
    pub ssl_server_name: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(typed_builder::TypedBuilder)]
#[builder(field_defaults(default, setter(into)))]
#[derive(enclave_protos_macros::ProtoBuilder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerOptions {
    #[prost(string, optional, tag="1")]
    pub bind_address: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint32, optional, tag="2")]
    pub port: ::core::option::Option<u32>,
    #[prost(string, optional, tag="3")]
    pub tls_key: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="4")]
    pub tls_key_path: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="5")]
    pub tls_pem: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="6")]
    pub tls_pem_path: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bool, optional, tag="7")]
    pub accept_http1: ::core::option::Option<bool>,
    #[prost(bool, optional, tag="8")]
    pub enable_web: ::core::option::Option<bool>,
    #[prost(uint64, optional, tag="9")]
    pub concurrency_limit_per_connection: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag="10")]
    pub timeout_ms: ::core::option::Option<u64>,
    #[prost(uint32, optional, tag="11")]
    pub initial_stream_window_size: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="12")]
    pub initial_connection_window_size: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="13")]
    pub max_concurrent_streams: ::core::option::Option<u32>,
    #[prost(uint64, optional, tag="14")]
    pub http2_keepalive_interval_ms: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag="15")]
    pub http2_keepalive_timeout_ms: ::core::option::Option<u64>,
    #[prost(bool, optional, tag="16")]
    pub http2_adaptive_window: ::core::option::Option<bool>,
    #[prost(uint64, optional, tag="17")]
    pub tcp_keepalive_ms: ::core::option::Option<u64>,
    #[prost(bool, optional, tag="18")]
    pub tcp_nodelay: ::core::option::Option<bool>,
    #[prost(uint32, optional, tag="19")]
    pub max_frame_size: ::core::option::Option<u32>,
}
/// Encoded file descriptor set for the `vaultron.service.v1` package
pub const FILE_DESCRIPTOR_SET: &[u8] = &[
    0x0a, 0xd1, 0x07, 0x0a, 0x20, 0x76, 0x61, 0x75, 0x6c, 0x74, 0x72, 0x6f, 0x6e, 0x2f, 0x73, 0x65,
    0x72, 0x76, 0x69, 0x63, 0x65, 0x2f, 0x76, 0x31, 0x2f, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x2e,
    0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x13, 0x76, 0x61, 0x75, 0x6c, 0x74, 0x72, 0x6f, 0x6e, 0x2e,
    0x73, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x2e, 0x76, 0x31, 0x22, 0xa3, 0x02, 0x0a, 0x0d, 0x43,
    0x6c, 0x69, 0x65, 0x6e, 0x74, 0x4f, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x12, 0x17, 0x0a, 0x04,
    0x68, 0x6f, 0x73, 0x74, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x48, 0x00, 0x52, 0x04, 0x68, 0x6f,
    0x73, 0x74, 0x88, 0x01, 0x01, 0x12, 0x17, 0x0a, 0x04, 0x70, 0x6f, 0x72, 0x74, 0x18, 0x02, 0x20,
    0x01, 0x28, 0x0d, 0x48, 0x01, 0x52, 0x04, 0x70, 0x6f, 0x72, 0x74, 0x88, 0x01, 0x01, 0x12, 0x1a,
    0x0a, 0x06, 0x69, 0x73, 0x5f, 0x73, 0x73, 0x6c, 0x18, 0x03, 0x20, 0x01, 0x28, 0x08, 0x48, 0x02,
    0x52, 0x05, 0x69, 0x73, 0x53, 0x73, 0x6c, 0x88, 0x01, 0x01, 0x12, 0x1e, 0x0a, 0x08, 0x73, 0x73,
    0x6c, 0x5f, 0x63, 0x65, 0x72, 0x74, 0x18, 0x04, 0x20, 0x01, 0x28, 0x09, 0x48, 0x03, 0x52, 0x07,
    0x73, 0x73, 0x6c, 0x43, 0x65, 0x72, 0x74, 0x88, 0x01, 0x01, 0x12, 0x27, 0x0a, 0x0d, 0x73, 0x73,
    0x6c, 0x5f, 0x63, 0x65, 0x72, 0x74, 0x5f, 0x70, 0x61, 0x74, 0x68, 0x18, 0x05, 0x20, 0x01, 0x28,
    0x09, 0x48, 0x04, 0x52, 0x0b, 0x73, 0x73, 0x6c, 0x43, 0x65, 0x72, 0x74, 0x50, 0x61, 0x74, 0x68,
    0x88, 0x01, 0x01, 0x12, 0x2b, 0x0a, 0x0f, 0x73, 0x73, 0x6c, 0x5f, 0x73, 0x65, 0x72, 0x76, 0x65,
    0x72, 0x5f, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x06, 0x20, 0x01, 0x28, 0x09, 0x48, 0x05, 0x52, 0x0d,
    0x73, 0x73, 0x6c, 0x53, 0x65, 0x72, 0x76, 0x65, 0x72, 0x4e, 0x61, 0x6d, 0x65, 0x88, 0x01, 0x01,
    0x42, 0x07, 0x0a, 0x05, 0x5f, 0x68, 0x6f, 0x73, 0x74, 0x42, 0x07, 0x0a, 0x05, 0x5f, 0x70, 0x6f,
    0x72, 0x74, 0x42, 0x09, 0x0a, 0x07, 0x5f, 0x69, 0x73, 0x5f, 0x73, 0x73, 0x6c, 0x42, 0x0b, 0x0a,
    0x09, 0x5f, 0x73, 0x73, 0x6c, 0x5f, 0x63, 0x65, 0x72, 0x74, 0x42, 0x10, 0x0a, 0x0e, 0x5f, 0x73,
    0x73, 0x6c, 0x5f, 0x63, 0x65, 0x72, 0x74, 0x5f, 0x70, 0x61, 0x74, 0x68, 0x42, 0x12, 0x0a, 0x10,
    0x5f, 0x73, 0x73, 0x6c, 0x5f, 0x73, 0x65, 0x72, 0x76, 0x65, 0x72, 0x5f, 0x6e, 0x61, 0x6d, 0x65,
    0x42, 0x94, 0x01, 0x0a, 0x17, 0x63, 0x6f, 0x6d, 0x2e, 0x76, 0x61, 0x75, 0x6c, 0x74, 0x72, 0x6f,
    0x6e, 0x2e, 0x73, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x2e, 0x76, 0x31, 0x42, 0x0b, 0x43, 0x6c,
    0x69, 0x65, 0x6e, 0x74, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x50, 0x01, 0xa2, 0x02, 0x03, 0x56, 0x53,
    0x58, 0xaa, 0x02, 0x13, 0x56, 0x61, 0x75, 0x6c, 0x74, 0x72, 0x6f, 0x6e, 0x2e, 0x53, 0x65, 0x72,
    0x76, 0x69, 0x63, 0x65, 0x2e, 0x56, 0x31, 0xca, 0x02, 0x13, 0x56, 0x61, 0x75, 0x6c, 0x74, 0x72,
    0x6f, 0x6e, 0x5c, 0x53, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x5c, 0x56, 0x31, 0xe2, 0x02, 0x1f,
    0x56, 0x61, 0x75, 0x6c, 0x74, 0x72, 0x6f, 0x6e, 0x5c, 0x53, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65,
    0x5c, 0x56, 0x31, 0x5c, 0x47, 0x50, 0x42, 0x4d, 0x65, 0x74, 0x61, 0x64, 0x61, 0x74, 0x61, 0xea,
    0x02, 0x15, 0x56, 0x61, 0x75, 0x6c, 0x74, 0x72, 0x6f, 0x6e, 0x3a, 0x3a, 0x53, 0x65, 0x72, 0x76,
    0x69, 0x63, 0x65, 0x3a, 0x3a, 0x56, 0x31, 0x4a, 0xd2, 0x03, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00,
    0x0b, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x0c, 0x12, 0x03, 0x00, 0x00, 0x12, 0x0a, 0x08, 0x0a, 0x01,
    0x02, 0x12, 0x03, 0x02, 0x00, 0x1c, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x04, 0x00,
    0x0b, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x04, 0x08, 0x15, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x05, 0x02, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x00, 0x04, 0x12, 0x03, 0x05, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x00, 0x05, 0x12, 0x03, 0x05, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x05, 0x12, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03,
    0x05, 0x24, 0x25, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x06, 0x02, 0x26,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x04, 0x12, 0x03, 0x06, 0x02, 0x0a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x05, 0x12, 0x03, 0x06, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x06, 0x12, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x01, 0x03, 0x12, 0x03, 0x06, 0x24, 0x25, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x02,
    0x12, 0x03, 0x07, 0x02, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x04, 0x12, 0x03,
    0x07, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x05, 0x12, 0x03, 0x07, 0x0b,
    0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x07, 0x12, 0x18, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x03, 0x12, 0x03, 0x07, 0x24, 0x25, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x00, 0x02, 0x03, 0x12, 0x03, 0x08, 0x02, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x03, 0x04, 0x12, 0x03, 0x08, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03,
    0x05, 0x12, 0x03, 0x08, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x01, 0x12,
    0x03, 0x08, 0x12, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x03, 0x12, 0x03, 0x08,
    0x24, 0x25, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x04, 0x12, 0x03, 0x09, 0x02, 0x26, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x04, 0x12, 0x03, 0x09, 0x02, 0x0a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x04, 0x05, 0x12, 0x03, 0x09, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x04, 0x01, 0x12, 0x03, 0x09, 0x12, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x04, 0x03, 0x12, 0x03, 0x09, 0x24, 0x25, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x05, 0x12,
    0x03, 0x0a, 0x02, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x04, 0x12, 0x03, 0x0a,
    0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x05, 0x12, 0x03, 0x0a, 0x0b, 0x11,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x01, 0x12, 0x03, 0x0a, 0x12, 0x21, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x03, 0x12, 0x03, 0x0a, 0x24, 0x25, 0x62, 0x06, 0x70, 0x72,
    0x6f, 0x74, 0x6f, 0x33, 0x0a, 0xe3, 0x16, 0x0a, 0x20, 0x76, 0x61, 0x75, 0x6c, 0x74, 0x72, 0x6f,
    0x6e, 0x2f, 0x73, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x2f, 0x76, 0x31, 0x2f, 0x73, 0x65, 0x72,
    0x76, 0x65, 0x72, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x13, 0x76, 0x61, 0x75, 0x6c, 0x74,
    0x72, 0x6f, 0x6e, 0x2e, 0x73, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x2e, 0x76, 0x31, 0x22, 0xb4,
    0x0a, 0x0a, 0x0d, 0x53, 0x65, 0x72, 0x76, 0x65, 0x72, 0x4f, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x73,
    0x12, 0x26, 0x0a, 0x0c, 0x62, 0x69, 0x6e, 0x64, 0x5f, 0x61, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73,
    0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x48, 0x00, 0x52, 0x0b, 0x62, 0x69, 0x6e, 0x64, 0x41, 0x64,
    0x64, 0x72, 0x65, 0x73, 0x73, 0x88, 0x01, 0x01, 0x12, 0x17, 0x0a, 0x04, 0x70, 0x6f, 0x72, 0x74,
    0x18, 0x02, 0x20, 0x01, 0x28, 0x0d, 0x48, 0x01, 0x52, 0x04, 0x70, 0x6f, 0x72, 0x74, 0x88, 0x01,
    0x01, 0x12, 0x1c, 0x0a, 0x07, 0x74, 0x6c, 0x73, 0x5f, 0x6b, 0x65, 0x79, 0x18, 0x03, 0x20, 0x01,
    0x28, 0x09, 0x48, 0x02, 0x52, 0x06, 0x74, 0x6c, 0x73, 0x4b, 0x65, 0x79, 0x88, 0x01, 0x01, 0x12,
    0x25, 0x0a, 0x0c, 0x74, 0x6c, 0x73, 0x5f, 0x6b, 0x65, 0x79, 0x5f, 0x70, 0x61, 0x74, 0x68, 0x18,
    0x04, 0x20, 0x01, 0x28, 0x09, 0x48, 0x03, 0x52, 0x0a, 0x74, 0x6c, 0x73, 0x4b, 0x65, 0x79, 0x50,
    0x61, 0x74, 0x68, 0x88, 0x01, 0x01, 0x12, 0x1c, 0x0a, 0x07, 0x74, 0x6c, 0x73, 0x5f, 0x70, 0x65,
    0x6d, 0x18, 0x05, 0x20, 0x01, 0x28, 0x09, 0x48, 0x04, 0x52, 0x06, 0x74, 0x6c, 0x73, 0x50, 0x65,
    0x6d, 0x88, 0x01, 0x01, 0x12, 0x25, 0x0a, 0x0c, 0x74, 0x6c, 0x73, 0x5f, 0x70, 0x65, 0x6d, 0x5f,
    0x70, 0x61, 0x74, 0x68, 0x18, 0x06, 0x20, 0x01, 0x28, 0x09, 0x48, 0x05, 0x52, 0x0a, 0x74, 0x6c,
    0x73, 0x50, 0x65, 0x6d, 0x50, 0x61, 0x74, 0x68, 0x88, 0x01, 0x01, 0x12, 0x26, 0x0a, 0x0c, 0x61,
    0x63, 0x63, 0x65, 0x70, 0x74, 0x5f, 0x68, 0x74, 0x74, 0x70, 0x31, 0x18, 0x07, 0x20, 0x01, 0x28,
    0x08, 0x48, 0x06, 0x52, 0x0b, 0x61, 0x63, 0x63, 0x65, 0x70, 0x74, 0x48, 0x74, 0x74, 0x70, 0x31,
    0x88, 0x01, 0x01, 0x12, 0x22, 0x0a, 0x0a, 0x65, 0x6e, 0x61, 0x62, 0x6c, 0x65, 0x5f, 0x77, 0x65,
    0x62, 0x18, 0x08, 0x20, 0x01, 0x28, 0x08, 0x48, 0x07, 0x52, 0x09, 0x65, 0x6e, 0x61, 0x62, 0x6c,
    0x65, 0x57, 0x65, 0x62, 0x88, 0x01, 0x01, 0x12, 0x4c, 0x0a, 0x20, 0x63, 0x6f, 0x6e, 0x63, 0x75,
    0x72, 0x72, 0x65, 0x6e, 0x63, 0x79, 0x5f, 0x6c, 0x69, 0x6d, 0x69, 0x74, 0x5f, 0x70, 0x65, 0x72,
    0x5f, 0x63, 0x6f, 0x6e, 0x6e, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x18, 0x09, 0x20, 0x01, 0x28,
    0x04, 0x48, 0x08, 0x52, 0x1d, 0x63, 0x6f, 0x6e, 0x63, 0x75, 0x72, 0x72, 0x65, 0x6e, 0x63, 0x79,
    0x4c, 0x69, 0x6d, 0x69, 0x74, 0x50, 0x65, 0x72, 0x43, 0x6f, 0x6e, 0x6e, 0x65, 0x63, 0x74, 0x69,
    0x6f, 0x6e, 0x88, 0x01, 0x01, 0x12, 0x22, 0x0a, 0x0a, 0x74, 0x69, 0x6d, 0x65, 0x6f, 0x75, 0x74,
    0x5f, 0x6d, 0x73, 0x18, 0x0a, 0x20, 0x01, 0x28, 0x04, 0x48, 0x09, 0x52, 0x09, 0x74, 0x69, 0x6d,
    0x65, 0x6f, 0x75, 0x74, 0x4d, 0x73, 0x88, 0x01, 0x01, 0x12, 0x40, 0x0a, 0x1a, 0x69, 0x6e, 0x69,
    0x74, 0x69, 0x61, 0x6c, 0x5f, 0x73, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x5f, 0x77, 0x69, 0x6e, 0x64,
    0x6f, 0x77, 0x5f, 0x73, 0x69, 0x7a, 0x65, 0x18, 0x0b, 0x20, 0x01, 0x28, 0x0d, 0x48, 0x0a, 0x52,
    0x17, 0x69, 0x6e, 0x69, 0x74, 0x69, 0x61, 0x6c, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x57, 0x69,
    0x6e, 0x64, 0x6f, 0x77, 0x53, 0x69, 0x7a, 0x65, 0x88, 0x01, 0x01, 0x12, 0x48, 0x0a, 0x1e, 0x69,
    0x6e, 0x69, 0x74, 0x69, 0x61, 0x6c, 0x5f, 0x63, 0x6f, 0x6e, 0x6e, 0x65, 0x63, 0x74, 0x69, 0x6f,
    0x6e, 0x5f, 0x77, 0x69, 0x6e, 0x64, 0x6f, 0x77, 0x5f, 0x73, 0x69, 0x7a, 0x65, 0x18, 0x0c, 0x20,
    0x01, 0x28, 0x0d, 0x48, 0x0b, 0x52, 0x1b, 0x69, 0x6e, 0x69, 0x74, 0x69, 0x61, 0x6c, 0x43, 0x6f,
    0x6e, 0x6e, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x57, 0x69, 0x6e, 0x64, 0x6f, 0x77, 0x53, 0x69,
    0x7a, 0x65, 0x88, 0x01, 0x01, 0x12, 0x39, 0x0a, 0x16, 0x6d, 0x61, 0x78, 0x5f, 0x63, 0x6f, 0x6e,
    0x63, 0x75, 0x72, 0x72, 0x65, 0x6e, 0x74, 0x5f, 0x73, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x73, 0x18,
    0x0d, 0x20, 0x01, 0x28, 0x0d, 0x48, 0x0c, 0x52, 0x14, 0x6d, 0x61, 0x78, 0x43, 0x6f, 0x6e, 0x63,
    0x75, 0x72, 0x72, 0x65, 0x6e, 0x74, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x73, 0x88, 0x01, 0x01,
    0x12, 0x42, 0x0a, 0x1b, 0x68, 0x74, 0x74, 0x70, 0x32, 0x5f, 0x6b, 0x65, 0x65, 0x70, 0x61, 0x6c,
    0x69, 0x76, 0x65, 0x5f, 0x69, 0x6e, 0x74, 0x65, 0x72, 0x76, 0x61, 0x6c, 0x5f, 0x6d, 0x73, 0x18,
    0x0e, 0x20, 0x01, 0x28, 0x04, 0x48, 0x0d, 0x52, 0x18, 0x68, 0x74, 0x74, 0x70, 0x32, 0x4b, 0x65,
    0x65, 0x70, 0x61, 0x6c, 0x69, 0x76, 0x65, 0x49, 0x6e, 0x74, 0x65, 0x72, 0x76, 0x61, 0x6c, 0x4d,
    0x73, 0x88, 0x01, 0x01, 0x12, 0x40, 0x0a, 0x1a, 0x68, 0x74, 0x74, 0x70, 0x32, 0x5f, 0x6b, 0x65,
    0x65, 0x70, 0x61, 0x6c, 0x69, 0x76, 0x65, 0x5f, 0x74, 0x69, 0x6d, 0x65, 0x6f, 0x75, 0x74, 0x5f,
    0x6d, 0x73, 0x18, 0x0f, 0x20, 0x01, 0x28, 0x04, 0x48, 0x0e, 0x52, 0x17, 0x68, 0x74, 0x74, 0x70,
    0x32, 0x4b, 0x65, 0x65, 0x70, 0x61, 0x6c, 0x69, 0x76, 0x65, 0x54, 0x69, 0x6d, 0x65, 0x6f, 0x75,
    0x74, 0x4d, 0x73, 0x88, 0x01, 0x01, 0x12, 0x37, 0x0a, 0x15, 0x68, 0x74, 0x74, 0x70, 0x32, 0x5f,
    0x61, 0x64, 0x61, 0x70, 0x74, 0x69, 0x76, 0x65, 0x5f, 0x77, 0x69, 0x6e, 0x64, 0x6f, 0x77, 0x18,
    0x10, 0x20, 0x01, 0x28, 0x08, 0x48, 0x0f, 0x52, 0x13, 0x68, 0x74, 0x74, 0x70, 0x32, 0x41, 0x64,
    0x61, 0x70, 0x74, 0x69, 0x76, 0x65, 0x57, 0x69, 0x6e, 0x64, 0x6f, 0x77, 0x88, 0x01, 0x01, 0x12,
    0x2d, 0x0a, 0x10, 0x74, 0x63, 0x70, 0x5f, 0x6b, 0x65, 0x65, 0x70, 0x61, 0x6c, 0x69, 0x76, 0x65,
    0x5f, 0x6d, 0x73, 0x18, 0x11, 0x20, 0x01, 0x28, 0x04, 0x48, 0x10, 0x52, 0x0e, 0x74, 0x63, 0x70,
    0x4b, 0x65, 0x65, 0x70, 0x61, 0x6c, 0x69, 0x76, 0x65, 0x4d, 0x73, 0x88, 0x01, 0x01, 0x12, 0x24,
    0x0a, 0x0b, 0x74, 0x63, 0x70, 0x5f, 0x6e, 0x6f, 0x64, 0x65, 0x6c, 0x61, 0x79, 0x18, 0x12, 0x20,
    0x01, 0x28, 0x08, 0x48, 0x11, 0x52, 0x0a, 0x74, 0x63, 0x70, 0x4e, 0x6f, 0x64, 0x65, 0x6c, 0x61,
    0x79, 0x88, 0x01, 0x01, 0x12, 0x29, 0x0a, 0x0e, 0x6d, 0x61, 0x78, 0x5f, 0x66, 0x72, 0x61, 0x6d,
    0x65, 0x5f, 0x73, 0x69, 0x7a, 0x65, 0x18, 0x13, 0x20, 0x01, 0x28, 0x0d, 0x48, 0x12, 0x52, 0x0c,
    0x6d, 0x61, 0x78, 0x46, 0x72, 0x61, 0x6d, 0x65, 0x53, 0x69, 0x7a, 0x65, 0x88, 0x01, 0x01, 0x42,
    0x0f, 0x0a, 0x0d, 0x5f, 0x62, 0x69, 0x6e, 0x64, 0x5f, 0x61, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73,
    0x42, 0x07, 0x0a, 0x05, 0x5f, 0x70, 0x6f, 0x72, 0x74, 0x42, 0x0a, 0x0a, 0x08, 0x5f, 0x74, 0x6c,
    0x73, 0x5f, 0x6b, 0x65, 0x79, 0x42, 0x0f, 0x0a, 0x0d, 0x5f, 0x74, 0x6c, 0x73, 0x5f, 0x6b, 0x65,
    0x79, 0x5f, 0x70, 0x61, 0x74, 0x68, 0x42, 0x0a, 0x0a, 0x08, 0x5f, 0x74, 0x6c, 0x73, 0x5f, 0x70,
    0x65, 0x6d, 0x42, 0x0f, 0x0a, 0x0d, 0x5f, 0x74, 0x6c, 0x73, 0x5f, 0x70, 0x65, 0x6d, 0x5f, 0x70,
    0x61, 0x74, 0x68, 0x42, 0x0f, 0x0a, 0x0d, 0x5f, 0x61, 0x63, 0x63, 0x65, 0x70, 0x74, 0x5f, 0x68,
    0x74, 0x74, 0x70, 0x31, 0x42, 0x0d, 0x0a, 0x0b, 0x5f, 0x65, 0x6e, 0x61, 0x62, 0x6c, 0x65, 0x5f,
    0x77, 0x65, 0x62, 0x42, 0x23, 0x0a, 0x21, 0x5f, 0x63, 0x6f, 0x6e, 0x63, 0x75, 0x72, 0x72, 0x65,
    0x6e, 0x63, 0x79, 0x5f, 0x6c, 0x69, 0x6d, 0x69, 0x74, 0x5f, 0x70, 0x65, 0x72, 0x5f, 0x63, 0x6f,
    0x6e, 0x6e, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x42, 0x0d, 0x0a, 0x0b, 0x5f, 0x74, 0x69, 0x6d,
    0x65, 0x6f, 0x75, 0x74, 0x5f, 0x6d, 0x73, 0x42, 0x1d, 0x0a, 0x1b, 0x5f, 0x69, 0x6e, 0x69, 0x74,
    0x69, 0x61, 0x6c, 0x5f, 0x73, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x5f, 0x77, 0x69, 0x6e, 0x64, 0x6f,
    0x77, 0x5f, 0x73, 0x69, 0x7a, 0x65, 0x42, 0x21, 0x0a, 0x1f, 0x5f, 0x69, 0x6e, 0x69, 0x74, 0x69,
    0x61, 0x6c, 0x5f, 0x63, 0x6f, 0x6e, 0x6e, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x5f, 0x77, 0x69,
    0x6e, 0x64, 0x6f, 0x77, 0x5f, 0x73, 0x69, 0x7a, 0x65, 0x42, 0x19, 0x0a, 0x17, 0x5f, 0x6d, 0x61,
    0x78, 0x5f, 0x63, 0x6f, 0x6e, 0x63, 0x75, 0x72, 0x72, 0x65, 0x6e, 0x74, 0x5f, 0x73, 0x74, 0x72,
    0x65, 0x61, 0x6d, 0x73, 0x42, 0x1e, 0x0a, 0x1c, 0x5f, 0x68, 0x74, 0x74, 0x70, 0x32, 0x5f, 0x6b,
    0x65, 0x65, 0x70, 0x61, 0x6c, 0x69, 0x76, 0x65, 0x5f, 0x69, 0x6e, 0x74, 0x65, 0x72, 0x76, 0x61,
    0x6c, 0x5f, 0x6d, 0x73, 0x42, 0x1d, 0x0a, 0x1b, 0x5f, 0x68, 0x74, 0x74, 0x70, 0x32, 0x5f, 0x6b,
    0x65, 0x65, 0x70, 0x61, 0x6c, 0x69, 0x76, 0x65, 0x5f, 0x74, 0x69, 0x6d, 0x65, 0x6f, 0x75, 0x74,
    0x5f, 0x6d, 0x73, 0x42, 0x18, 0x0a, 0x16, 0x5f, 0x68, 0x74, 0x74, 0x70, 0x32, 0x5f, 0x61, 0x64,
    0x61, 0x70, 0x74, 0x69, 0x76, 0x65, 0x5f, 0x77, 0x69, 0x6e, 0x64, 0x6f, 0x77, 0x42, 0x13, 0x0a,
    0x11, 0x5f, 0x74, 0x63, 0x70, 0x5f, 0x6b, 0x65, 0x65, 0x70, 0x61, 0x6c, 0x69, 0x76, 0x65, 0x5f,
    0x6d, 0x73, 0x42, 0x0e, 0x0a, 0x0c, 0x5f, 0x74, 0x63, 0x70, 0x5f, 0x6e, 0x6f, 0x64, 0x65, 0x6c,
    0x61, 0x79, 0x42, 0x11, 0x0a, 0x0f, 0x5f, 0x6d, 0x61, 0x78, 0x5f, 0x66, 0x72, 0x61, 0x6d, 0x65,
    0x5f, 0x73, 0x69, 0x7a, 0x65, 0x42, 0x94, 0x01, 0x0a, 0x17, 0x63, 0x6f, 0x6d, 0x2e, 0x76, 0x61,
    0x75, 0x6c, 0x74, 0x72, 0x6f, 0x6e, 0x2e, 0x73, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x2e, 0x76,
    0x31, 0x42, 0x0b, 0x53, 0x65, 0x72, 0x76, 0x65, 0x72, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x50, 0x01,
    0xa2, 0x02, 0x03, 0x56, 0x53, 0x58, 0xaa, 0x02, 0x13, 0x56, 0x61, 0x75, 0x6c, 0x74, 0x72, 0x6f,
    0x6e, 0x2e, 0x53, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x2e, 0x56, 0x31, 0xca, 0x02, 0x13, 0x56,
    0x61, 0x75, 0x6c, 0x74, 0x72, 0x6f, 0x6e, 0x5c, 0x53, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x5c,
    0x56, 0x31, 0xe2, 0x02, 0x1f, 0x56, 0x61, 0x75, 0x6c, 0x74, 0x72, 0x6f, 0x6e, 0x5c, 0x53, 0x65,
    0x72, 0x76, 0x69, 0x63, 0x65, 0x5c, 0x56, 0x31, 0x5c, 0x47, 0x50, 0x42, 0x4d, 0x65, 0x74, 0x61,
    0x64, 0x61, 0x74, 0x61, 0xea, 0x02, 0x15, 0x56, 0x61, 0x75, 0x6c, 0x74, 0x72, 0x6f, 0x6e, 0x3a,
    0x3a, 0x53, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x3a, 0x3a, 0x56, 0x31, 0x4a, 0xd3, 0x0a, 0x0a,
    0x06, 0x12, 0x04, 0x00, 0x00, 0x18, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x0c, 0x12, 0x03, 0x00, 0x00,
    0x12, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x02, 0x00, 0x1c, 0x0a, 0x0a, 0x0a, 0x02, 0x04,
    0x00, 0x12, 0x04, 0x04, 0x00, 0x18, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03,
    0x04, 0x08, 0x15, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x05, 0x02, 0x37,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x04, 0x12, 0x03, 0x05, 0x02, 0x0a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x05, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x05, 0x12, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x00, 0x03, 0x12, 0x03, 0x05, 0x35, 0x36, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01,
    0x12, 0x03, 0x06, 0x02, 0x37, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x04, 0x12, 0x03,
    0x06, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x05, 0x12, 0x03, 0x06, 0x0b,
    0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x06, 0x12, 0x16, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x06, 0x35, 0x36, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x00, 0x02, 0x02, 0x12, 0x03, 0x07, 0x02, 0x37, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x02, 0x04, 0x12, 0x03, 0x07, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02,
    0x05, 0x12, 0x03, 0x07, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12,
    0x03, 0x07, 0x12, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x03, 0x12, 0x03, 0x07,
    0x35, 0x36, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x03, 0x12, 0x03, 0x08, 0x02, 0x37, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x04, 0x12, 0x03, 0x08, 0x02, 0x0a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x03, 0x05, 0x12, 0x03, 0x08, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x03, 0x01, 0x12, 0x03, 0x08, 0x12, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x03, 0x03, 0x12, 0x03, 0x08, 0x35, 0x36, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x04, 0x12,
    0x03, 0x09, 0x02, 0x37, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x04, 0x12, 0x03, 0x09,
    0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x05, 0x12, 0x03, 0x09, 0x0b, 0x11,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x01, 0x12, 0x03, 0x09, 0x12, 0x19, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x03, 0x12, 0x03, 0x09, 0x35, 0x36, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x00, 0x02, 0x05, 0x12, 0x03, 0x0a, 0x02, 0x37, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x05, 0x04, 0x12, 0x03, 0x0a, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x05,
    0x12, 0x03, 0x0a, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x01, 0x12, 0x03,
    0x0a, 0x12, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x03, 0x12, 0x03, 0x0a, 0x35,
    0x36, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x06, 0x12, 0x03, 0x0b, 0x02, 0x37, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x06, 0x04, 0x12, 0x03, 0x0b, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x06, 0x05, 0x12, 0x03, 0x0b, 0x0b, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x06, 0x01, 0x12, 0x03, 0x0b, 0x12, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x06,
    0x03, 0x12, 0x03, 0x0b, 0x35, 0x36, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x07, 0x12, 0x03,
    0x0c, 0x02, 0x37, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x07, 0x04, 0x12, 0x03, 0x0c, 0x02,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x07, 0x05, 0x12, 0x03, 0x0c, 0x0b, 0x0f, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x07, 0x01, 0x12, 0x03, 0x0c, 0x12, 0x1c, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x07, 0x03, 0x12, 0x03, 0x0c, 0x35, 0x36, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x00, 0x02, 0x08, 0x12, 0x03, 0x0d, 0x02, 0x37, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x08,
    0x04, 0x12, 0x03, 0x0d, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x08, 0x05, 0x12,
    0x03, 0x0d, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x08, 0x01, 0x12, 0x03, 0x0d,
    0x12, 0x32, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x08, 0x03, 0x12, 0x03, 0x0d, 0x35, 0x36,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x09, 0x12, 0x03, 0x0e, 0x02, 0x38, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x09, 0x04, 0x12, 0x03, 0x0e, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x09, 0x05, 0x12, 0x03, 0x0e, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x09, 0x01, 0x12, 0x03, 0x0e, 0x12, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x09, 0x03,
    0x12, 0x03, 0x0e, 0x35, 0x37, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x0a, 0x12, 0x03, 0x0f,
    0x02, 0x38, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0a, 0x04, 0x12, 0x03, 0x0f, 0x02, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0a, 0x05, 0x12, 0x03, 0x0f, 0x0b, 0x11, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x0a, 0x01, 0x12, 0x03, 0x0f, 0x12, 0x2c, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x0a, 0x03, 0x12, 0x03, 0x0f, 0x35, 0x37, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00,
    0x02, 0x0b, 0x12, 0x03, 0x10, 0x02, 0x38, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0b, 0x04,
    0x12, 0x03, 0x10, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0b, 0x05, 0x12, 0x03,
    0x10, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0b, 0x01, 0x12, 0x03, 0x10, 0x12,
    0x30, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0b, 0x03, 0x12, 0x03, 0x10, 0x35, 0x37, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x0c, 0x12, 0x03, 0x11, 0x02, 0x38, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x0c, 0x04, 0x12, 0x03, 0x11, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x0c, 0x05, 0x12, 0x03, 0x11, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0c,
    0x01, 0x12, 0x03, 0x11, 0x12, 0x28, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0c, 0x03, 0x12,
    0x03, 0x11, 0x35, 0x37, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x0d, 0x12, 0x03, 0x12, 0x02,
    0x38, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0d, 0x04, 0x12, 0x03, 0x12, 0x02, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0d, 0x05, 0x12, 0x03, 0x12, 0x0b, 0x11, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x0d, 0x01, 0x12, 0x03, 0x12, 0x12, 0x2d, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x0d, 0x03, 0x12, 0x03, 0x12, 0x35, 0x37, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02,
    0x0e, 0x12, 0x03, 0x13, 0x02, 0x38, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0e, 0x04, 0x12,
    0x03, 0x13, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0e, 0x05, 0x12, 0x03, 0x13,
    0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0e, 0x01, 0x12, 0x03, 0x13, 0x12, 0x2c,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0e, 0x03, 0x12, 0x03, 0x13, 0x35, 0x37, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x00, 0x02, 0x0f, 0x12, 0x03, 0x14, 0x02, 0x38, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x0f, 0x04, 0x12, 0x03, 0x14, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x0f, 0x05, 0x12, 0x03, 0x14, 0x0b, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0f, 0x01,
    0x12, 0x03, 0x14, 0x12, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0f, 0x03, 0x12, 0x03,
    0x14, 0x35, 0x37, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x10, 0x12, 0x03, 0x15, 0x02, 0x38,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x10, 0x04, 0x12, 0x03, 0x15, 0x02, 0x0a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x10, 0x05, 0x12, 0x03, 0x15, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x10, 0x01, 0x12, 0x03, 0x15, 0x12, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x10, 0x03, 0x12, 0x03, 0x15, 0x35, 0x37, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x11,
    0x12, 0x03, 0x16, 0x02, 0x38, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x11, 0x04, 0x12, 0x03,
    0x16, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x11, 0x05, 0x12, 0x03, 0x16, 0x0b,
    0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x11, 0x01, 0x12, 0x03, 0x16, 0x12, 0x1d, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x11, 0x03, 0x12, 0x03, 0x16, 0x35, 0x37, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x00, 0x02, 0x12, 0x12, 0x03, 0x17, 0x02, 0x38, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x12, 0x04, 0x12, 0x03, 0x17, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x12,
    0x05, 0x12, 0x03, 0x17, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x12, 0x01, 0x12,
    0x03, 0x17, 0x12, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x12, 0x03, 0x12, 0x03, 0x17,
    0x35, 0x37, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
];
include!("vaultron.service.v1.serde.rs");
// @@protoc_insertion_point(module)