// This file is @generated by prost-build.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct TlsInspector {
    #[prost(message, optional, tag = "1")]
    pub enable_ja3_fingerprinting: ::core::option::Option<
        super::super::super::super::super::super::google::protobuf::BoolValue,
    >,
    #[prost(message, optional, tag = "2")]
    pub initial_read_buffer_size: ::core::option::Option<
        super::super::super::super::super::super::google::protobuf::UInt32Value,
    >,
}
